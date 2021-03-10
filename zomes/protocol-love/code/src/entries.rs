


//need element: Entry + Header

#[hdk_extern(id = "entry", visibility = "public", required_validations = 10)] //public entry with ex 10 valid
#derive(Debug, Clone, Serialize, Deserialize, Serializedbytes)]
pub enum Entry {
	PublishEntry,
	UpdateEntry,
	DeleteEntry,
}

#[hdk_extern]
pub fn CreateEntry(PublishEntry: Entry) -> ExternResult<PublishEntry> {
        let mut commit_entry: PublishEntry = Entry::new();
        let Entry: Entries = zome_info!()?.zome_Entries();
        commit_entry.insert((Entry, "commit_entry".into()));
    Ok(create_entry!(Entry {
        tag: "".into(),
        access: commit.into(),
        functions
    })?)
}
//need validation callback

/* 
#[hdk_extern]
fn validate_entry (PublishEntry: Entry) -> 
ExternResult<ValidateEntry> { 
if let Ok(_) = Entry::try_from() {  //Hash Address Base ?
Ok(ValidateEntry::Valid) 
}
*/
