CREATE INDEX assets__ft_contract_id_idx ON assets__fungible_token_events (emitted_by_contract_account_id);
CREATE INDEX assets__nft_contract_id_idx ON assets__non_fungible_token_events (emitted_by_contract_account_id);
