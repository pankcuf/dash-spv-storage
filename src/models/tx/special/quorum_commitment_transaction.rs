use crate::schema::quorum_commitment_transactions;

#[derive(Identifiable, Queryable, PartialEq, Eq, Debug)]
pub struct QuorumCommitmentTransaction {
    pub id: i32,
    pub base_id: i32,
    pub quorum_id: i32,
    pub quorum_commitment_height: i32,
}

#[derive(Insertable, PartialEq, Eq, Debug)]
#[table_name="quorum_commitment_transactions"]
pub struct NewQuorumCommitmentTransaction {
    pub base_id: i32,
    pub quorum_id: i32,
    pub quorum_commitment_height: i32,
}
