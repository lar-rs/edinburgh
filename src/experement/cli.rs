


pub fn hello()  {
    use ansi_term::Colour;

    println!(r#" {:} "#,Colour::Blue.paint("  ███████╗██████╗ ██╗███╗   ██╗██████╗ ██╗   ██╗██████╗  ██████╗ ██╗  ██╗   "));
    println!(r#" {:} "#,Colour::Blue.paint("  ██╔════╝██╔══██╗██║████╗  ██║██╔══██╗██║   ██║██╔══██╗██╔════╝ ██║  ██║   "));
    println!(r#" {:} "#,Colour::Blue.paint("  █████╗  ██║  ██║██║██╔██╗ ██║██████╔╝██║   ██║██████╔╝██║  ███╗███████║   "));
    println!(r#" {:} "#,Colour::Blue.paint("  ██╔══╝  ██║  ██║██║██║╚██╗██║██╔══██╗██║   ██║██╔══██╗██║   ██║██╔══██║   "));
    println!(r#" {:} "#,Colour::Blue.paint("  ███████╗██████╔╝██║██║ ╚████║██████╔╝╚██████╔╝██║  ██║╚██████╔╝██║  ██║   "));
    println!(r#" {:} "#,Colour::Blue.paint("  ╚══════╝╚═════╝ ╚═╝╚═╝  ╚═══╝╚═════╝  ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝   "));
    println!(r#" {:} "#,Colour::Blue.paint("   NDIR sensor driver  "));
    // println!(r#" "#);
    // println!(r#" "#);
    // println!(" {:}  ",Colour::Blue.paint(format!("Number of logical cores is {}",num_cpus::get())));
    // println!(r#"  ENVIRONMENTAL MONITORING  "#);
}
