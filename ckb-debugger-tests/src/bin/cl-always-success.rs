use ckb_debugger_tests::combine_lock_mol::ChildScript;
use ckb_debugger_tests::{create_script_from_cell_dep, create_simple_case, read_tx_template};
use ckb_mock_tx_types::ReprMockTransaction;
use ckb_types::prelude::Pack;
use molecule::prelude::{Builder, Entity};

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tx = read_tx_template("../ckb-debugger-tests/templates/cl-always-success.json")?;
    let mut repr_tx = ReprMockTransaction::from(tx.clone());

    let child_script = create_script_from_cell_dep(&repr_tx, 1, true)?;
    let child_script = child_script.as_builder().args([].pack()).build();
    let child_script: ChildScript = child_script.into();

    let (smt_root, witness_args) = create_simple_case(
        vec![child_script.clone(), child_script.clone(), child_script],
        1,
    );

    let mut args = vec![0x00u8];
    args.extend(smt_root.as_slice());

    repr_tx.mock_info.inputs[0].output.lock.args = ckb_jsonrpc_types::JsonBytes::from_vec(args);
    repr_tx.tx.witnesses[0] = ckb_jsonrpc_types::JsonBytes::from(witness_args.pack());

    let json = serde_json::to_string_pretty(&repr_tx).unwrap();
    println!("{}", json);
    Ok(())
}
