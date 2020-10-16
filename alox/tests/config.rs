extern crate alox;

use std::{
    result::Result as StdResult,
    error::Error,
    fs::File,
    io::Read
};

use alox::{
    config::{
        AloxConfig
    },
    toml::from_str
};

type Result<T> = StdResult<T, Box<dyn std::error::Error>>;


#[test]
fn test_config_alox() -> Result<()> {
    println!("CWD: {}", std::env::current_dir().unwrap().to_str().unwrap());
    let mut file = File::open("../test-data/alox.toml")?;
    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)?;
    let alox_config: AloxConfig = from_str(&file_contents)?;
    println!("{:#?}", alox_config);
    Ok(())
}

#[test]
fn test_config_cms_block() -> Result<()> {
    use alox::{
        cms::{
            block::{
                Block,
                BlockParamType
            }
        },
        ron::from_str as ron_from_str,
        serde_json::{
            to_string_pretty,
            from_str
        }
    };

    let toml = "
        ident = \"text_block\"
        template = \"blocks/text_block.html\"

        [params.title]
        type = \"text\"

        [params.content]
        type = \"row\"
        
            [[params.content.column_types]]
            type = \"asset\"

            [[params.content.column_types]]
            type = \"richtext\"
    ";

    let ron = "
        (
            ident: \"text_block\",
            template: \"blocks/text_block.html\",
            params: {
                \"title\": text,
                \"image\": asset,
                \"content\": row([
                    asset,
                    list(text)
                ])
            }
        )   
    ";

    //let block: Block = from_str(toml)?;
    let mut block: Block = ron_from_str(ron)?;
    let block_json = to_string_pretty(&block)?;
    block = from_str(&block_json)?;

    println!("{:#?}", block);
    println!("{}", block_json);

    Ok(())
}