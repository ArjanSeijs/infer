open Core
open Textuallib
module F = Format

(* Root dir for .ullbc files *)
let program_root = "./programs"

(* Read file to string *)
let read_file path = In_channel.read_all path

(* Normalize: drop CRs and trim *)
let normalize s =
  let buf = Buffer.create (String.length s) in
  String.iter s ~f:(fun c ->
      if not (Char.equal c '\r') then Buffer.add_char buf c);
  String.strip (Buffer.contents buf)

(* Translate <stem>.ullbc to a Textual string *)
let translate_stem stem =
  let ullbc = Filename.concat program_root (stem ^ ".ullbc") in
  let json = Yojson.Basic.from_file ullbc in
  match Charon.UllbcOfJson.crate_of_json json with
  | Error e -> failwith e
  | Ok crate ->
      let m =
        RustFrontend.RustMir2Textual.mk_module crate (stem ^ ".ullbc")
      in
      F.asprintf "%a@?" (Textual.Module.pp ~show_location:false) m

(* Recursively list stems (paths without .ullbc) under start_rel *)
let list_stems_under start_rel =
  let start_abs =
    if String.is_empty start_rel then program_root
    else Filename.concat program_root start_rel
  in
  let rec walk ~rel_dir ~abs_dir acc =
    Stdlib.Sys.readdir abs_dir
    |> Array.fold ~init:acc ~f:(fun acc name ->
           let abs = Filename.concat abs_dir name in
           let rel =
             if String.is_empty rel_dir then name else Filename.concat rel_dir name
           in
           if Stdlib.Sys.is_directory abs then
             walk ~rel_dir:rel ~abs_dir:abs acc
           else if Filename.check_suffix name ".ullbc" then
             let base =
               String.sub name ~pos:0
                 ~len:(String.length name - String.length ".ullbc")
             in
             let stem =
               if String.is_empty rel_dir then base
               else Filename.concat rel_dir base
             in
             stem :: acc
           else acc)
  in
  if not (Stdlib.Sys.file_exists start_abs) || not (Stdlib.Sys.is_directory start_abs)
  then []
  else List.sort ~compare:String.compare (walk ~rel_dir:start_rel ~abs_dir:start_abs [])

(* Show unified diff using temp files *)
let show_console_diff ~expected ~actual =
  let write_tmp ~prefix ~suffix ~data =
    let path = Stdlib.Filename.temp_file prefix suffix in
    let oc = Out_channel.create path in
    Out_channel.output_string oc data;
    Out_channel.close oc;
    path
  in
  let tmp_exp = write_tmp ~prefix:"expected" ~suffix:".sil" ~data:expected in
  let tmp_act = write_tmp ~prefix:"actual" ~suffix:".sil" ~data:actual in
  ignore (Stdlib.Sys.command (Printf.sprintf "diff -u %s %s" tmp_exp tmp_act));
  Stdlib.Sys.remove tmp_exp;
  Stdlib.Sys.remove tmp_act

(* Console helpers *)
let print_separator () =
  print_endline (String.make 70 '-')

(* Print status block: OK | MISS | DIFF | ERR *)
let print_block ~status ~stem ?details () =
  print_endline "";
  Printf.printf "%-5s %s\n" status stem;
  print_separator ();
  Option.iter details ~f:(fun d -> Printf.printf "%s\n" d)

(* RUN_UNDER filter; empty = all *)
let get_under_arg () =
  match Stdlib.Sys.getenv_opt "RUN_UNDER" with
  | Some s when not (String.is_empty s) -> s
  | _ -> ""

(* Walk stems, translate, diff, summarize; exit non-zero on issues *)
let () =
  let under = get_under_arg () in
  let stems = list_stems_under under in
  if List.is_empty stems then (
    let where =
      if String.is_empty under then program_root
      else Filename.concat program_root under
    in
    prerr_endline (Printf.sprintf "no .ullbc files found under: %s" where);
    exit 1
  );

  let ok_count = ref 0
  and miss_count = ref 0
  and diff_count = ref 0
  and err_count = ref 0 in

  List.iter stems ~f:(fun stem ->
      try
        let translated_raw = translate_stem stem in
        let actual_norm = normalize translated_raw in
        let expected_path = Filename.concat program_root (stem ^ ".sil") in
        if not (Stdlib.Sys.file_exists expected_path) then (
          incr miss_count;
          print_block ~status:"MISS" ~stem
            ~details:(Printf.sprintf "(missing %s)" expected_path) ()
        ) else
          let expected_raw = read_file expected_path in
          let expected_norm = normalize expected_raw in
          if String.equal actual_norm expected_norm then (
            incr ok_count;
            print_block ~status:"OK" ~stem ()
          ) else (
            incr diff_count;
            print_block ~status:"DIFF" ~stem ();
            show_console_diff ~expected:expected_raw ~actual:translated_raw
          )
      with exn ->
        incr err_count;
        print_block ~status:"ERR" ~stem ~details:(Exn.to_string exn) ()
    );

  print_endline (String.make 70 '=');
  Printf.printf "Summary: OK=%d  MISS=%d  DIFF=%d  ERR=%d\n"
    !ok_count !miss_count !diff_count !err_count;

  if !diff_count + !miss_count + !err_count > 0 then exit 1
