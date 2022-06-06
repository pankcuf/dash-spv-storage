use chrono::NaiveDateTime;
use dash_spv_primitives::crypto::UInt256;
use crate::schema::friend_requests;

/// queries:
/// "sourceContact == %@"
/// "destinationContact == %@"
/// "destinationContact.associatedBlockchainIdentity.uniqueID == %@"
/// "sourceContact.associatedBlockchainIdentity.uniqueID == %@"
/// "sourceContact.associatedBlockchainIdentity.uniqueID == %@ && destinationContact.associatedBlockchainIdentity.uniqueID == %@"
/// "derivationPath.publicKeyIdentifier == %@"
/// "sourceContact == %@ && destinationContact.associatedBlockchainIdentity.uniqueID == %@"
/// "destinationContact == %@ && sourceContact.associatedBlockchainIdentity.uniqueID == %@"
/// "(derivationPath.chain == %@)"
/// "(friendshipIdentifier == %@)"
/// "sourceContact == %@ && (SUBQUERY(sourceContact.incomingRequests, $friendRequest, $friendRequest.sourceContact == SELF.destinationContact).@count == 0)"
/// "destinationContact == %@ && (SUBQUERY(destinationContact.outgoingRequests, $friendRequest, $friendRequest.destinationContact == SELF.sourceContact).@count > 0)"
/// indexation:
/// "destinationContact.associatedBlockchainIdentity.dashpayUsername.stringValue"
#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct FriendRequest {
    pub id: i32,
    pub account_id: i32,
    pub source_key_index: i32,
    pub destination_key_index: i32,
    pub source_contact_id: i32,
    pub destination_contact_id: i32,
    pub derivation_path_id: i32,
    pub timestamp: NaiveDateTime,//i64
    pub friendship_identifier: UInt256,

}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="friend_requests"]
pub struct NewFriendRequest {
    pub account_id: i32,
    pub source_key_index: i32,
    pub destination_key_index: i32,
    pub source_contact_id: i32,
    pub destination_contact_id: i32,
    pub derivation_path_id: i32,
    pub timestamp: NaiveDateTime,//i64
    pub friendship_identifier: UInt256,
}
