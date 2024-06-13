use crate::{generators::common::{indent, make_defined_types_fields}, types::Event, IDL};
use convert_case::{Casing, Case};

pub fn make_events(idl: &IDL) -> String {
    idl.events.iter().map(|event| {
        let event_name_pascal = event.name.to_case(Case::Pascal);

        let fields = match event.fields.is_empty() {
            true => if let Some(matched_type) = idl.types.iter().find(|t| t.name == event_name_pascal) {
                        indent(make_defined_types_fields(matched_type.clone()))
                    } else {
                        make_event_props(event)
                    },
            false => make_event_props(event)
        };
        
        format!("#[cfg_attr(not(target_os=\"solana\"), derive(Debug))]
#[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
pub struct {} {{
{}
}}
    
impl anchor_lang::Event for {} {{
    fn data(&self) -> Vec<u8> {{
        let mut data = Self::DISCRIMINATOR.to_vec();
        self.serialize(&mut data).unwrap();
        data
    }}
}}", event_name_pascal, indent(fields), event_name_pascal)
    }).collect::<Vec<String>>().join("\n\n")
}

pub fn make_event_props(event: &Event) -> String {
    event.fields.iter().map(|t| format!("        pub {}: {}", t.name.to_case(Case::Snake), t.kind.to_string())).collect::<Vec<String>>().join(",\n")
}