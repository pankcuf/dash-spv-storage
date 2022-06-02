table! {
    local_masternodes (id) {
        id -> Integer,
        holding_keys_index -> Nullable<Integer>,
        holding_keys_wallet_id -> Nullable<Text>,
        operator_keys_index -> Nullable<Integer>,
        operator_keys_wallet_id -> Nullable<Text>,
        owner_keys_index -> Nullable<Integer>,
        owner_keys_wallet_id -> Nullable<Text>,
        voting_keys_index -> Nullable<Integer>,
        voting_keys_wallet_id -> Nullable<Text>,
        provider_registration_transaction -> Nullable<Integer>,
        masternode_id -> Nullable<Integer>,
    }
}

table! {
    masternode_addresses (address_id, masternode_id) {
        address_id -> Integer,
        masternode_id -> Integer,
    }
}

table! {
    masternode_list_masternodes (masternode_list_id, masternode_id) {
        masternode_list_id -> Integer,
        masternode_id -> Integer,
    }
}

table! {
    masternode_list_quorums (masternode_list_id, quorum_id) {
        masternode_list_id -> Integer,
        quorum_id -> Integer,
    }
}

table! {
    masternode_lists (id) {
        id -> Integer,
        block_id -> Integer,
        chain_id -> Integer,
        masternodes_merkle_root -> Nullable<Binary>,
        quorums_merkle_root -> Nullable<Binary>,
    }
}

table! {
    masternodes (id) {
        id -> Integer,
        chain_id -> Integer,
        address -> BigInt,
        port -> SmallInt,
        core_last_connection_date -> Nullable<Timestamp>,
        core_protocol -> BigInt,
        core_version -> Nullable<Text>,
        is_valid -> Bool,
        platform_ping -> BigInt,
        platform_ping_date -> Nullable<Timestamp>,
        platform_version -> Nullable<Text>,
        known_confirmed_at_height -> Integer,
        update_height -> Integer,
        local_masternode_id -> Nullable<Integer>,
        prev_operator_bls_public_keys -> Nullable<Binary>,
        prev_masternode_entry_hashes -> Nullable<Binary>,
        prev_validity -> Nullable<Binary>,
        confirmed_hash -> Binary,
        ipv6_address -> Binary,
        key_id_voting -> Binary,
        operator_bls_public_key -> Binary,
        provider_registration_transaction_hash -> Binary,
        masternode_entry_hash -> Binary,
    }
}

table! {
    quorums (id) {
        id -> Integer,
        block_id -> Integer,
        chain_id -> Integer,
        verified -> Bool,
        version -> SmallInt,
        all_commitment_aggregated_signature -> Binary,
        commitment_hash -> Binary,
        commitment_transaction_id -> Nullable<Integer>,
        quorum_index -> Nullable<Integer>,
        quorum_type -> SmallInt,
        quorum_hash -> Binary,
        quorum_public_key -> Binary,
        quorum_threshold_signature -> Binary,
        quorum_verification_vector_hash -> Binary,
        signers_count -> Integer,
        signers_bitset -> Binary,
        valid_members_count -> Integer,
        valid_members_bitset -> Binary,
    }
}

allow_tables_to_appear_in_same_query!(
    local_masternodes,
    masternode_addresses,
    masternode_list_masternodes,
    masternode_list_quorums,
    masternode_lists,
    masternodes,
    quorums,
);
