use std::io::{self,BufRead};

use regex::Regex;

#[derive(Debug)]
enum Datatype{
    Int,
    Nvarchar,
    Datetime,
    Datetime2,
    Decimal,
    Tinyint,
    Bit,
    None,
}

#[derive(Debug)]
struct Items{
    field: String,
    datatype: Datatype,
    optional: bool
}

fn main() {
    let stdin = io::stdin();
    let mut lines_vec:Vec<String> = Vec::new();
    for line in stdin.lock().lines(){
        if let Ok(l) = line{
            lines_vec.push(l);
        }
    }
    let buffer = lines_vec.join("\n");

    //convert sql create statement to rust struct
    
    //extract create fields
    /*
CREATE TABLE [dbo].[mandantor](
	[idmandantor] [int] IDENTITY(1,1) NOT NULL,
	[idintegration] [int] NULL,
	[idsenderaddress] [int] NULL,
	[name] [nvarchar](135) NOT NULL,
	[description] [nvarchar](450) NULL,
	[deleted] [bit] NOT NULL,
	[editedby] [int] NOT NULL,
	[SysStartTime] [datetime2](7) GENERATED ALWAYS AS ROW START NOT NULL,
	[SysEndTime] [datetime2](7) GENERATED ALWAYS AS ROW END NOT NULL,
	[privatekey] [nvarchar](450) NULL,
	[publickey] [nvarchar](450) NULL,
	[email] [nvarchar](450) NULL,
	[needsreturnlabel] [bit] NOT NULL,
	PERIOD FOR SYSTEM_TIME ([SysStartTime], [SysEndTime])        
*/

    let re = Regex::new(r"(CREATE TABLE \[dbo\]\.\[([a-zA-Z_\-]+)\]\()").unwrap();
    let re_fields = Regex::new(r"\[([a-zA-Z_\-]+)\] \[([a-zA-Z_\-]+)\]").unwrap();
    let mut struct_name = "".to_string();
    let mut replace = "".to_string();
    for matches in re.captures_iter(&buffer){
        //println!("create matches{:#?}",matches);
        struct_name = matches[2].to_string();
        replace = matches[1].to_string();
    }
    let buffer = &buffer.replace(&replace, "");
    let mut struct_items:Vec<Items> = Vec::new();
    let _test = buffer
        .split("\n\t")
        .into_iter()
        .filter(|s|!s.is_empty())
        .map(|itm|{ 
          for matches in re_fields.captures_iter(itm){
            if !matches[1].is_empty() && !matches[2].is_empty(){
                let mut optional=true;
                if itm.contains("NOT NULL"){
                    optional = false;
                }
                let field = &matches[1].to_string();
                let datatype = match &matches[2]{
                    "nvarchar" => Datatype::Nvarchar,
                    "int" => Datatype::Int,
                    "datetime2" => Datatype::Datetime2,
                    "datetime" => Datatype::Datetime,
                    "decimal" => Datatype::Decimal,
                    "tinyint" => Datatype::Tinyint,
                    "bit" => Datatype::Bit,
                    _=> Datatype::None
                };


                let struct_item = Items{
                    field: field.to_string(),
                    datatype,
                    optional,
                };
                struct_items.push(struct_item);
            }
          }
          itm
        }).collect::<Vec<&str>>();

    let output = format!(r"
    #[derive(Debug,Serialize,Deserialize)]
    pub struct {}{{ {}     
    }}
    ",struct_name.to_string().uppercase_first(),get_output_fields(&struct_items));
    println!("{}",output);
}

fn get_output_fields(struct_items: &Vec<Items>)-> String{
    let mut output = String::new();
    for itm in struct_items{
       let mut datatype = match itm.datatype{
            Datatype::Int => "i32".to_string(),
            Datatype::Nvarchar => "String".to_string(),
            Datatype::Datetime => "chrono::NaiveDateTime".to_string(),
            Datatype::Datetime2 => "chrono::NaiveDateTime".to_string(),
            Datatype::Decimal => "f64".to_string(),
            Datatype::Tinyint => "bool".to_string(),
            Datatype::Bit => "bool".to_string(),
            _ => "".to_string(),
        };
        if itm.optional{
            datatype = format!("Option<{}>",datatype);
        }
       output = format!("{}\n\tpub {}:{},",output,itm.field,datatype);
    }
    output
}

trait FirstToUpper{
    fn uppercase_first(&self)->Self;
}

impl FirstToUpper for String{
    fn uppercase_first(&self)->Self{
        let mut v: Vec<char> = self.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        let s2: String = v.into_iter().collect();
        let s3 = &s2;
        s3.to_string()
    }
}
