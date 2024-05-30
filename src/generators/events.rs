use crate::{types::Event, IDL};
use convert_case::{Casing, Case};

pub fn make_events(idl: &IDL) -> String {
    idl.events.iter().map(|event| {
        let event_name_pascal = event.name.to_case(Case::Pascal);
        
        format!("    #[derive(AnchorDiscriminator, AnchorSerialize, AnchorDeserialize)]
    pub struct {} {{
{}
    }}
    
    impl anchor_lang::Event for {} {{
        fn data(&self) -> Vec<u8> {{
            let mut data = Self::DISCRIMINATOR.to_vec();
            self.serialize(&mut data).unwrap();
            data
        }}
    }}", event_name_pascal, make_event_props(event), event_name_pascal)
    }).collect::<Vec<String>>().join("\n\n")
}

pub fn make_event_props(event: &Event) -> String {
    event.fields.iter().map(|t| format!("        pub {}: {}", t.name.to_case(Case::Snake), t.kind.to_string())).collect::<Vec<String>>().join(",\n")
}