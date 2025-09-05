use std::process::Command;

fn main()
{
   let output = Command::new("rustc")
      .arg("--version")
      .output()
      .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

   if output.status.success()
   {
      let captured_out = String::from_utf8_lossy(&output.stdout);
      print!("rustc succeeded and stdout was:\n{}", captured_out);
   }
   else
   {
      let captured_out = String::from_utf8_lossy(&output.stderr);
      print!("rustc failed and stderr was:\n{}", captured_out);
   }
}
