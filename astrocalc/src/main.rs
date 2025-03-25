use clap::{Arg, Command};
const PI:f32 = std::f32::consts::PI;
use unicode_width::UnicodeWidthStr;
use regex::{Match, Regex};

fn strip_ansi(s: &str) -> String {
    let ansi_regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    ansi_regex.replace_all(s, "").to_string()
}

fn gen_box(lines: &[String]) -> String {
    let max_width = lines.iter()
        .map(|s| UnicodeWidthStr::width(strip_ansi(s).as_str()))
        .max()
        .unwrap_or(0);

    let mut output = String::new();
    output.push_str(&format!("┏{}┓\n", "━".repeat(max_width)));

    for line in lines {
        let visible_width = UnicodeWidthStr::width(strip_ansi(line).as_str());
        let padding = max_width - visible_width;
        output.push_str(&format!("┃{}{}┃\n", line, " ".repeat(padding)));
    }

    output.push_str(&format!("┗{}┛", "━".repeat(max_width)));
    output
}


fn calc_distance(isdeg:String, showwork:String, a1:f32, d1:f32, a2:f32, d2:f32) {
    macro_rules! verbose {
        ($($arg:tt)*) => {
            if showwork == "true" {
                println!($($arg)*);
            }
        };
    }
    let mut a1 = a1;
    let mut d1 = d1;
    let mut a2 = a2;
    let mut d2 = d2;
    verbose!("α₁ = {}", a1);
    verbose!("δ₁ = {}", d1);
    verbose!("α₂ = {}", a2);
    verbose!("δ₂ = {}", d2);

    if isdeg == "true" {
        verbose!("Convert values into radians");
        verbose!("n = n * π/180");
       
        verbose!("\tα₁ = {} * π/180 = \x1b[1m\x1b[37m{}\x1b[0m", a1, a1 * (PI / 180.0));
        a1 = a1 * (PI / 180.0);

        verbose!("\tδ₁ = {} * π/180 = \x1b[1m\x1b[37m{}\x1b[0m", d1, d1 * (PI / 180.0));
        d1 = d1 * (PI / 180.0);

        verbose!("\tα₂ = {} * π/180 = \x1b[1m\x1b[37m{}\x1b[0m", a2, a2 * (PI / 180.0));
        a2 = a2 * (PI / 180.0);

        verbose!("\tδ₂ = {} * π/180 = \x1b[1m\x1b[37m{}\x1b[0m", d2, d2 * (PI / 180.0));
        d2 = d2 * (PI / 180.0);
    }
    verbose!("Apply Spherical law of Cosines");
    verbose!("cos(θ) = sin(δ₁)sin(δ₂) + cos(δ₁)cos(δ₂)cos(α₂ - α₁)");
    verbose!("\tsin(δ₁)sin(δ₂) = \x1b[1m\x1b[37m{}\x1b[0m", d1.sin() * d2.sin());
    verbose!("\tcos(δ₁)cos(δ₂)cos(α₂ - α₁) = \x1b[1m\x1b[37m{}\x1b[0m", d1.cos() * d2.cos() * (a2 - a1).cos());
    verbose!("\tcos(θ) = \x1b[1m\x1b[37m{}\x1b[0m", (d1.sin() * d2.sin()) + (d1.cos() * d2.cos() * (a2 - a1).cos()));
    
    let cos_theta:f32 = (d1.sin() * d2.sin()) + (d1.cos() * d2.cos() * (a2 - a1).cos());
    verbose!("θ = cos⁻¹(θ)");
    let theta:f32 = cos_theta.acos();
    
    
    // Convert to degrees, arcminutes, arcseconds
    verbose!("Convert θ to degrees, arcminutes, and arcseconds");
    verbose!("θ° = θ * 180/π");
    let theta_deg = theta * (180.0 / PI);
    verbose!("\tθ° = {} * 180/π = \x1b[1m\x1b[37m{}\x1b[0m°", theta, theta_deg);
    
    let theta_arcmin = (theta_deg - theta_deg.floor()) * 60.0;
    let theta_arcsec = (theta_arcmin - theta_arcmin.floor()) * 60.0;
    
    verbose!("Calculate arcminutes and arcseconds");
    verbose!("θ' = (θ° - floor(θ°)) * 60");
    verbose!("\tθ' = ({} - floor({})) * 60 = \x1b[1m\x1b[37m{}\x1b[0m'", theta_deg, theta_deg, theta_arcmin.floor());
    
    verbose!("θ\" = (θ' - floor(θ')) * 60");
    verbose!("\tθ\" = ({} - floor({})) * 60 = \x1b[1m\x1b[37m{}\x1b[0m", theta_arcmin, theta_arcmin, theta_arcsec);
    let theta_output_rad = format!("θ = \x1b[1m\x1b[37m{}\x1b[0m rad", theta);

let theta_output_deg = format!(
    "θ = \x1b[1m\x1b[37m{}°{}′{}″\x1b[0m",
    theta_deg.floor(),
    theta_arcmin.floor(),
    theta_arcsec
);

    print!("{}",gen_box(&[theta_output_rad, theta_output_deg]));
    // α δ θ ₁ ₂ π ″

}

fn convert(conversion: String, input: String) {
    if conversion == "deg-rad" {
        let deg: f32 = input.trim().parse().expect("Invalid degree value");
        let coeff = deg / 180.0;
        print!("{}",gen_box(&[format!("{}π rad", coeff)]));
    } else if conversion == "rad-deg" {
        if input.contains("pi") {
            let coeff_str = input.replace("pi", "").trim().to_string();
            let coeff: f32 = if coeff_str.is_empty() {
                1.0
            } else {
                coeff_str.parse().expect("Invalid coefficient for pi")
            };
            let deg = coeff * 180.0;
            print!("{}",gen_box(&[format!("{}°", deg)]));
        } else {
            let rad: f32 = input.trim().parse().expect("Invalid radian value");
            let deg = rad * 180.0 / PI;
            print!("{}",gen_box(&[format!("{}°", deg)]));
        }
    } else {
        print!("{}", gen_box(&[format!("Invalid conversion type")]))
    }
}

fn main() {
    let matches = Command::new("Astrocalc")
        .version("1.0")
        .author("beanfrog")
        .about("perform astronomical calculations and conversions")
        .subcommand(
            Command::new("distance")
                .about("calculate approximate distance or angular separation between 2 Ra/Dec points")
                .alias("d")
                .arg(
                    Arg::new("isdeg")
                        .required(true)
                        .help("Are the following values in degrees? (false = radians)"),
                )
                .arg(
                    Arg::new("showwork")
                        .required(true)
                        .help("Show the calculations used?"),
                )
                .arg(
                    Arg::new("alpha1")
                        .required(true)
                        .value_parser(clap::value_parser!(f32))
                        .help("Ra of first coordinate"),
                )
                .arg(
                    Arg::new("delta1")
                        .required(true)
                        .value_parser(clap::value_parser!(f32))
                        .help("Dec of first coordinate"),
                )
                .arg(
                    Arg::new("alpha2")
                        .required(true)
                        .value_parser(clap::value_parser!(f32))
                        .help("Ra of second coordinate"),
                )
                .arg(
                    Arg::new("delta2")
                        .required(true)
                        .value_parser(clap::value_parser!(f32))
                        .help("Dec of second coordinate"),
                ),
        )
        .subcommand (
            Command::new("convert")
            .about("various conversions")
            .alias("c")
            .arg(
                Arg::new("conversion")
                .required(true)
                .help("rad-deg | deg-rad")
            )
            .arg (
                Arg::new("input")
                .required(true)
                .help("value to convert")
            )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("distance", args)) => {
            let isdeg = args.get_one::<String>("isdeg").unwrap();
            let showwork = args.get_one::<String>("showwork").unwrap();
            let alpha1 = args.get_one::<f32>("alpha1").unwrap();
            let delta1 = args.get_one::<f32>("delta1").unwrap();
            let alpha2 = args.get_one::<f32>("alpha2").unwrap();
            let delta2 = args.get_one::<f32>("delta2").unwrap();
            calc_distance(isdeg.clone(),showwork.clone(), *alpha1, *delta1, *alpha2, *delta2)
        }
        Some (("convert", args)) => {
            let conversion = args.get_one::<String>("conversion").unwrap();
            let input = args.get_one::<String>("input").unwrap();
            convert(conversion.clone(), input.clone());
        }
        _ => println!("No subcommand was used."),
    }
}
