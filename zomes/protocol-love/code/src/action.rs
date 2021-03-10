use hdk::holochain_json_api::{
	json::JsonString,
	error::JsonError,
};
use hdk::holochain_core_types::dna::entry_types::Sharing;
use holochain_wasm_utils::holochain_persistence_api::cas::content::Address;
use holochain_wasm_utils::holochain_core_types::entry::Entry;
use std::borrow::Borrow;
use hdk::error::ZomeApiResult;
use holochain_wasm_utils::holochain_core_types::link::LinkMatch;
use hdk::prelude::ValidatingEntryType;

/// An `Action` that updates the state in the CoGov system.
///
/// Every action that updates the state is represented by
/// the Action struct.
///
/// # Examples
///
/// ```
/// Action {
///		op: ActionOp::CreateCollective,
/// 	status: ActionStatus::Executed,
/// 	data: json!({"name": "My Collective", "admin_address": hdk::AGENT_ADDRESS.clone()}).into(),
/// 	prev_data: serde_json::value::Value::Null.into(),
/// 	tag: "create_collective".into(),
/// 	strategy: ActionStrategy::SystemAutomatic
/// }
/// ```
///
#[hdk_extern(id = "action")]
#[derive(Clone, Copy)]
pub struct Action {
	/// Represents each of the allowed operations
	pub op: ActionOp,
	/// Lifecycle Status of the Action
	pub status: ActionStatus,
	/// Action Data encoded as JSON.
	pub data: JsonString,
	/// Previous Action Data encoded as JSON for undo purposes.
	pub prev_data: JsonString,
	pub tag: String,
	/// How the Action was performed
	pub strategy: ActionStrategy,
}
#[hdk_extern(id = "address")]
#derive(Clone, Copy]
pub struct Address {
	pub address: Vec<Address>,
}

/// An operation for an [Action](struct.Action.html).
#[derive(Serialize, Deserialize, Debug, Serializedbytes, Clone)]
pub enum ActionOp {
	CreateCollective,
	AddCollectivePerson,
	SetCollectiveName,
}

/// The lifecycle status of an [Action](struct.Action.html).
#[derive(Serialize, Deserialize, Debug, Serializedbytes, Clone)]
pub enum ActionStatus {
	/// Action is currently opened & not completed
	Open,
	/// Action is executed & completed
	///
	/// TODO: Rename
	Executed,
}

/// How an [Action](struct.Action.html) is performed.
#[derive(Serialize, Deserialize, Debug, Serializedbytes, Clone)]
pub enum ActionStrategy {
	/// Performed via automation by the system
	SystemAutomatic,
	/// TODO: Evaluate
	PrivilegedAction,
	/// TODO: Evaluate
	NewDiscussionMessage,
}

/// A tuple containing an [Address](type.Address.html), [Entry](enum.Entry.html), & [Action](struct.Action.html)
pub type ActionEntry = (Address, Entry, Action);

// * might need to make self an address reference * //
pub trait RootAction {
	fn commit_action(self, collective_address: Address) -> ExternResult<ActionEntry>;
}

pub trait ChildAction {
	fn commit_action(self, collective_address: Address, parent_action_address: Address) -> ExternResult<ActionEntry>;
}

//todo
impl ChildAction for Action {
	fn commit_action(self, collective_address: Address, parent_action_address: Address) -> ZomeApiResult<ActionEntry> {
		let action_entry = Entry::App("action".into(), self.borrow().into());
		let action_address = hdk::commit_entry(&action_entry)?;
		hdk::link_entries(
			&collective_address,
			&action_address,
			"collective->action",
			"",
		)?;
		hdk::link_entries(
			&parent_action_address,
			&action_address,
			"child->action",
			"",
		)?;
		Ok((ActionAddress, ActionEntry, self))
	}
}

/// Returns a Holochain entry definition for an action. /* need to change */
pub fn action_def() -> ValidatingEntryType {
	entry!(
		name: "action",
		description: "A protocol.love collective action",
		sharing: Sharing::Public,
		validation_package: || {
			hdk::ValidationPackageDefinition::Entry
		},
		validation: | _validation_data: hdk::EntryValidationData<Action>| {
			Ok(())
		},
		links: [
			from!(
				"collective",
				link_type: "action_collective",
				validation_package: || {
					hdk::ValidationPackageDefinition::Entry
				},
				validation: |_validation_data: hdk::LinkValidationData| {
					Ok(())
				}
			),
			to!(
				"action",
				link_type: "child->action",
				validation_package: || {
					hdk::ValidationPackageDefinition::Entry
				},
				validation: |_validation_data: hdk::LinkValidationData| {
					Ok(())
				}
			)
		]
	)
}
/* */

entry_defs![Address::entry_def(), Entry::entry_def(), Action::entry_def()]