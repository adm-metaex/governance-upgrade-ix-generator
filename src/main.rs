use std::str::FromStr;

use base64::{engine::general_purpose, Engine};
use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    bpf_loader_upgradeable::set_upgrade_authority,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

/// InstructionData wrapper. It can be removed once Borsh serialization for
/// Instruction is supported in the SDK
#[derive(Clone, Debug, PartialEq, Eq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct InstructionData {
    /// Pubkey of the instruction processor that executes this instruction
    pub program_id: Pubkey,
    /// Metadata for what accounts should be passed to the instruction processor
    pub accounts: Vec<AccountMetaData>,
    /// Opaque data passed to the instruction processor
    pub data: Vec<u8>,
}

/// Account metadata used to define Instructions
#[derive(Clone, Debug, PartialEq, Eq, BorshDeserialize, BorshSerialize, BorshSchema)]
pub struct AccountMetaData {
    /// An account's public key
    pub pubkey: Pubkey,
    /// True if an Instruction requires a Transaction signature matching
    /// `pubkey`.
    pub is_signer: bool,
    /// True if the `pubkey` can be loaded as a read-write account.
    pub is_writable: bool,
}

impl From<Instruction> for InstructionData {
    fn from(instruction: Instruction) -> Self {
        InstructionData {
            program_id: instruction.program_id,
            accounts: instruction
                .accounts
                .iter()
                .map(|a| AccountMetaData {
                    pubkey: a.pubkey,
                    is_signer: a.is_signer,
                    is_writable: a.is_writable,
                })
                .collect(),
            data: instruction.data,
        }
    }
}

impl From<&InstructionData> for Instruction {
    fn from(instruction: &InstructionData) -> Self {
        Instruction {
            program_id: instruction.program_id,
            accounts: instruction
                .accounts
                .iter()
                .map(|a| AccountMeta {
                    pubkey: a.pubkey,
                    is_signer: a.is_signer,
                    is_writable: a.is_writable,
                })
                .collect(),
            data: instruction.data.clone(),
        }
    }
}

fn main() {
    // Arrange
    // let program_address = Pubkey::from_str("78sycjkMouQ2HJnpnvDUzgBCt81jMJVZMf5rLhZ5bgrh").unwrap();
    // let buffer_address = Pubkey::from_str("CjoWQim52bBVk9xZQJBoxwoiEcAHx68WTP8GrFKJdUKQ").unwrap();
    // in the current context, governance is the same as the upgrade authority of governance program
    // let governance = Pubkey::from_str("8Nm2CFjLx1Vnd1D1NvnCfdq3BJBzZ8aNRcCuTnhr7FVh").unwrap();

    let program_address = Pubkey::from_str("D9KEi2SGUuX71zgGYPBScScZagrm7J8jSEduBTF84xtj").unwrap();
    let authority = Pubkey::from_str("C6DmyYh1KXNMAvdMzP845aP2WhXkfmvu6qaC9kQReKLQ").unwrap();
    let new_authority = Pubkey::from_str("Bc1WrTZZUQyRQkKQNqcBqLpoxMQehx4mBXk3aVsJRxhp").unwrap();

    let transfer_instruction =
        set_upgrade_authority(&program_address, &authority, Some(&new_authority));

    // let upgrade_instruction = bpf_loader_upgradeable::upgrade(
    //     &program_address,
    //     &buffer_address,
    //     &governance,
    //     &governance,
    // );

    // Act
    let instruction_data: InstructionData = transfer_instruction.clone().into();
    let mut instruction_bytes = vec![];
    instruction_data.serialize(&mut instruction_bytes).unwrap();

    // base64 encoded message is accepted as the input in the UI
    let encoded = general_purpose::STANDARD_NO_PAD.encode(&instruction_bytes);

    // Assert
    let instruction =
        Instruction::from(&InstructionData::deserialize(&mut &instruction_bytes[..]).unwrap());

    assert_eq!(transfer_instruction, instruction);

    println!("Encoded ix: {}", encoded);
}
