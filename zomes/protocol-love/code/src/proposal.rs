use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};
//need to update:
use hdk::holochain_core_types::dna::entry_types::Sharing;
use holochain_wasm_utils::holochain_persistence_api::cas::content::Address;
use holochain_wasm_utils::holochain_core_types::entry::Entry;
use std::borrow::Borrow;
use hdk::error::ZomeApiResult;
use hdk::prelude::ValidatingEntryType;
//* */

/// A proposal to change the collective.
#[derive(Serialize, Deserialize, Debug, Serializedbytes, Clone)]
pub struct Proposal {
	/// Name of the proposal
	pub name: String,
	/// Text content of the proposal.
	pub content: String,
}
/// Api to create [Proposal](struct.Proposal.html).
#[hdk_extern(id = "create_proposal", visibility = "public", required validations = 10)] //public proposal with ex 10 valid
#[derive(Clone, Copy)]
pub fn create_proposal(name: Proposal, content: Proposal) -> ExternResult<Proposal> {
		let mut Proposal: name = create_entry!(Proposal::new(name)).App(App::Create))?;
	Ok(Proposal)
	}
}

//unclear how to use:
entry_def!(Proposal, EntryDef {
	id: proposal_name.into(//
		/* */
}

#[entrydefs!(vec![Proposal::entry_def()]