use fluentbase_codec::Encoder;
use fluentbase_poseidon::poseidon_hash;
use fluentbase_runtime::{
    ExecutionResult,
    Runtime,
    RuntimeContext,
    SysFuncIdx,
    STATE_DEPLOY,
    STATE_MAIN,
};
use fluentbase_rwasm::{
    instruction_set,
    rwasm::{Compiler, CompilerConfig, FuncOrExport},
};
use fluentbase_sdk::evm::ContractInput;
use hex_literal::hex;

fn wasm2rwasm(wasm_binary: &[u8], inject_fuel_consumption: bool) -> Vec<u8> {
    let import_linker = Runtime::<()>::new_linker();
    Compiler::new_with_linker(
        &wasm_binary.to_vec(),
        CompilerConfig::default().fuel_consume(inject_fuel_consumption),
        Some(&import_linker),
    )
    .unwrap()
    .finalize()
    .unwrap()
}

fn run_rwasm_with_evm_input(wasm_binary: Vec<u8>, input_data: &[u8]) -> ExecutionResult<()> {
    let input_data = {
        let mut contract_input = ContractInput::default();
        contract_input.contract_input = input_data.to_vec();
        contract_input.encode_to_vec(0)
    };
    let rwasm_binary = wasm2rwasm(wasm_binary.as_slice(), false);
    let ctx = RuntimeContext::new(rwasm_binary)
        .with_state(STATE_MAIN)
        .with_fuel_limit(100_000)
        .with_input(input_data)
        .with_catch_trap(true);
    let import_linker = Runtime::<()>::new_linker();
    let mut runtime = Runtime::<()>::new(ctx, &import_linker).unwrap();
    runtime.data_mut().clean_output();
    runtime.call().unwrap()
}

#[test]
fn test_greeting() {
    let output = run_rwasm_with_evm_input(
        include_bytes!("../../examples/bin/greeting.wasm").to_vec(),
        "Hello, World".as_bytes(),
    );
    assert_eq!(output.data().exit_code(), 0);
    assert_eq!(
        output.data().output().clone(),
        "Hello, World".as_bytes().to_vec()
    );
}

#[test]
fn test_keccak256() {
    let output = run_rwasm_with_evm_input(
        include_bytes!("../../examples/bin/keccak256.wasm").to_vec(),
        "Hello, World".as_bytes(),
    );
    assert_eq!(output.data().exit_code(), 0);
    assert_eq!(
        output.data().output().clone(),
        hex!("a04a451028d0f9284ce82243755e245238ab1e4ecf7b9dd8bf4734d9ecfd0529").to_vec()
    );
}

#[test]
fn test_poseidon() {
    let input_data = "Hello, World".as_bytes();
    let output = run_rwasm_with_evm_input(
        include_bytes!("../../examples/bin/poseidon.wasm").to_vec(),
        input_data,
    );
    assert_eq!(output.data().exit_code(), 0);
    assert_eq!(
        output.data().output().clone(),
        poseidon_hash(input_data).to_vec()
    );
}

#[test]
fn test_secp256k1_verify() {
    let wasm_binary = include_bytes!("../../examples/bin/secp256k1.wasm");
    let rwasm_binary = wasm2rwasm(wasm_binary, true);

    let input_datas: &[&[u8]] = &[
        &[
            173, 132, 205, 11, 16, 252, 2, 135, 56, 151, 27, 7, 129, 36, 174, 194, 160, 231, 198,
            217, 134, 163, 129, 190, 11, 56, 111, 50, 190, 232, 135, 175, 206, 83, 171, 179, 114,
            27, 175, 197, 97, 64, 140, 232, 255, 153, 201, 9, 247, 240, 177, 138, 47, 120, 134, 73,
            214, 71, 1, 98, 171, 26, 160, 50, 57, 113, 237, 197, 35, 166, 214, 69, 63, 63, 182, 18,
            141, 49, 141, 157, 177, 165, 255, 51, 134, 254, 177, 4, 125, 152, 22, 231, 128, 3, 157,
            82, 0, 2, 26, 122, 86, 158, 145, 219, 246, 5, 129, 80, 156, 127, 201, 70, 209, 0, 59,
            96, 199, 222, 232, 82, 153, 83, 141, 182, 53, 53, 56, 213, 149, 116,
        ],
        &[
            173, 132, 205, 11, 16, 252, 2, 135, 56, 151, 27, 7, 129, 36, 174, 194, 160, 231, 198,
            217, 134, 163, 129, 190, 11, 56, 111, 50, 190, 232, 135, 175, 70, 192, 91, 99, 104,
            164, 75, 136, 16, 215, 152, 89, 68, 29, 129, 155, 142, 124, 220, 139, 253, 55, 30, 53,
            197, 49, 150, 244, 188, 172, 219, 81, 53, 199, 250, 204, 226, 169, 123, 149, 234, 203,
            168, 165, 134, 216, 123, 121, 88, 170, 248, 54, 138, 178, 156, 238, 72, 31, 118, 232,
            113, 219, 217, 203, 1, 3, 109, 108, 170, 194, 72, 175, 150, 246, 175, 167, 249, 4, 245,
            80, 37, 58, 15, 62, 243, 245, 170, 47, 230, 131, 138, 149, 178, 22, 105, 20, 104, 226,
        ],
    ];

    for input_data in input_datas {
        let output =
            Runtime::<()>::run(rwasm_binary.as_slice(), &input_data.to_vec(), 10_000_000).unwrap();
        assert_eq!(output.data().output().clone(), Vec::<u8>::new());
    }
}

#[test]
fn test_panic() {
    let output = run_rwasm_with_evm_input(
        include_bytes!("../../examples/bin/panic.wasm").to_vec(),
        &[],
    );
    assert_eq!(output.data().exit_code(), -71);
}

#[test]
fn test_state() {
    let wasm_binary = include_bytes!("../../examples/bin/state.wasm");
    let import_linker = Runtime::<()>::new_linker();
    let mut compiler = Compiler::new_with_linker(
        wasm_binary.as_slice(),
        CompilerConfig::default()
            .fuel_consume(false)
            .translate_sections(true),
        Some(&import_linker),
    )
    .unwrap();
    compiler
        .translate(FuncOrExport::StateRouter(
            vec![FuncOrExport::Export("main"), FuncOrExport::Export("deploy")],
            instruction_set! {
                Call(SysFuncIdx::SYS_STATE)
            },
        ))
        .unwrap();
    let rwasm_bytecode = compiler.finalize().unwrap();
    let result = Runtime::<()>::run_with_context(
        RuntimeContext::new(rwasm_bytecode.clone())
            .with_state(STATE_DEPLOY)
            .with_fuel_limit(100_000),
        &import_linker,
    )
    .unwrap();
    assert_eq!(result.data().output()[0], 100);
    let result = Runtime::<()>::run_with_context(
        RuntimeContext::new(rwasm_bytecode)
            .with_state(STATE_MAIN)
            .with_fuel_limit(100_000),
        &import_linker,
    )
    .unwrap();
    assert_eq!(result.data().output()[0], 200);
}