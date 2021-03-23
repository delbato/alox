extern crate alox;

use std::{
    result::Result as StdResult,
    error::Error,
    fs::File,
    io::Read
};

use alox::{
    config::{
    },
    toml::from_str
};

type Result<T> = StdResult<T, Box<dyn std::error::Error>>;

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
            preview_template_str: Some(\"
                <table>
                    <tr>
                        <td>
                            blub!
                        </td>
                    </tr>
                </table>
            \"),
            params: {
                \"title\": text,
                \"image\": asset,
                \"order\": enum([\"forward\", \"backward\"]),
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