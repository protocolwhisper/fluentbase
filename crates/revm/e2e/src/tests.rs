use crate::runner::execute_test_suite;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn run_e2e_test(test_path: &'static str) {
    let path = if cfg!(target_os = "linux") {
        format!("./{}", test_path)
    } else {
        format!("../{}", test_path)
    };
    let elapsed = Arc::new(Mutex::new(Duration::new(0, 0)));
    execute_test_suite(Path::new(path.as_str()), &elapsed, false, false, None).unwrap();
}

macro_rules! define_tests {
    (
        $( fn $test_name:ident($test_path:literal); )*
    ) => {
        $(
            #[test]
            fn $test_name() {
                run_e2e_test($test_path)
            }
        )*
    };
}

mod e2e_tests {
    use super::*;

    define_tests! {

        // --- FAIL --- (bug with gas calculation, it affects other tests as well)
        // fn call_recursive_bomb_log2("tests/GeneralStateTests/stSystemOperationsTest/CallRecursiveBombLog2.json");

        // --- ALL PASS ---
        // fn sstore_combinations_initial01_2_paris("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial01_2_Paris.json");
        // fn sstore_combinations_initial00_paris("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial00_Paris.json");
        // fn sstore_combinations_initial11_paris("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial11_Paris.json");
        // fn sstore_combinations_initial00_2_paris("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial00_2_Paris.json");
        // fn static_call50000_sha256("tests/GeneralStateTests/stTimeConsuming/static_Call50000_sha256.json");
        // fn sstore_combinations_initial20_2("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial20_2.json");
        // fn sstore_combinations_initial00_2("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial00_2.json");
        // fn sstore_combinations_initial10_2("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial10_2.json");
        // fn sstore_combinations_initial21_paris("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial21_Paris.json");
        // fn call_blake2f_max_rounds("tests/GeneralStateTests/stTimeConsuming/CALLBlake2f_MaxRounds.json");
        // fn sstore_combinations_initial21("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial21.json");
        // fn sstore_combinations_initial01("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial01.json");
        // fn sstore_combinations_initial21_2("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial21_2.json");
        // fn sstore_combinations_initial01_paris("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial01_Paris.json");
        // fn sstore_combinations_initial00("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial00.json");
        // fn sstore_combinations_initial10_paris("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial10_Paris.json");
        // fn sstore_combinations_initial20("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial20.json");
        // fn sstore_combinations_initial01_2("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial01_2.json");
        // fn sstore_combinations_initial11("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial11.json");
        // fn sstore_combinations_initial20_paris("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial20_Paris.json");
        // fn sstore_combinations_initial10_2_paris("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial10_2_Paris.json");
        // fn sstore_combinations_initial21_2_paris("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial21_2_Paris.json");
        // fn sstore_combinations_initial11_2_paris("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial11_2_Paris.json");
        // fn sstore_combinations_initial11_2("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial11_2.json");
        // fn sstore_combinations_initial20_2_paris("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial20_2_Paris.json");
        // fn sstore_combinations_initial10("tests/GeneralStateTests/stTimeConsuming/sstore_combinations_initial10.json");

        // -- MOST PASS --- (failing marked with FAIL or CRASH comments, also gas calculation issue)
        // fn random_statetest16("tests/GeneralStateTests/stRandom/randomStatetest16.json");
        // fn random_statetest380("tests/GeneralStateTests/stRandom/randomStatetest380.json");
        // fn random_statetest103("tests/GeneralStateTests/stRandom/randomStatetest103.json");
        // fn random_statetest338("tests/GeneralStateTests/stRandom/randomStatetest338.json");
        // fn random_statetest41("tests/GeneralStateTests/stRandom/randomStatetest41.json");
        // fn random_statetest292("tests/GeneralStateTests/stRandom/randomStatetest292.json");
        fn random_statetest154("tests/GeneralStateTests/stRandom/randomStatetest154.json"); // FAIL
        // fn random_statetest57("tests/GeneralStateTests/stRandom/randomStatetest57.json");
        // fn random_statetest142("tests/GeneralStateTests/stRandom/randomStatetest142.json");
        // fn random_statetest379("tests/GeneralStateTests/stRandom/randomStatetest379.json");
        // fn random_statetest115("tests/GeneralStateTests/stRandom/randomStatetest115.json");
        // fn random_statetest247("tests/GeneralStateTests/stRandom/randomStatetest247.json");
        // fn random_statetest302("tests/GeneralStateTests/stRandom/randomStatetest302.json");
        // fn random_statetest210("tests/GeneralStateTests/stRandom/randomStatetest210.json");
        // fn random_statetest355("tests/GeneralStateTests/stRandom/randomStatetest355.json");
        // fn random_statetest139("tests/GeneralStateTests/stRandom/randomStatetest139.json");
        // fn random_statetest206("tests/GeneralStateTests/stRandom/randomStatetest206.json");
        // fn random_statetest343("tests/GeneralStateTests/stRandom/randomStatetest343.json");
        // fn random_statetest82("tests/GeneralStateTests/stRandom/randomStatetest82.json");
        // fn random_statetest251("tests/GeneralStateTests/stRandom/randomStatetest251.json");
        // fn random_statetest197("tests/GeneralStateTests/stRandom/randomStatetest197.json");
        fn random_statetest178("tests/GeneralStateTests/stRandom/randomStatetest178.json"); // FAIL
        // fn random_statetest363("tests/GeneralStateTests/stRandom/randomStatetest363.json");
        // fn random_statetest0("tests/GeneralStateTests/stRandom/randomStatetest0.json");
        // fn random_statetest226("tests/GeneralStateTests/stRandom/randomStatetest226.json");
        // fn random_statetest158("tests/GeneralStateTests/stRandom/randomStatetest158.json");
        // fn random_statetest334("tests/GeneralStateTests/stRandom/randomStatetest334.json");
        // fn random_statetest271("tests/GeneralStateTests/stRandom/randomStatetest271.json");
        // fn random_statetest322("tests/GeneralStateTests/stRandom/randomStatetest322.json");
        // fn random_statetest288("tests/GeneralStateTests/stRandom/randomStatetest288.json");
        // fn random_statetest267("tests/GeneralStateTests/stRandom/randomStatetest267.json");
        // fn random_statetest119("tests/GeneralStateTests/stRandom/randomStatetest119.json");
        // fn random_statetest230("tests/GeneralStateTests/stRandom/randomStatetest230.json");
        // fn random_statetest162("tests/GeneralStateTests/stRandom/randomStatetest162.json");
        // fn random_statetest98("tests/GeneralStateTests/stRandom/randomStatetest98.json");
        // fn random_statetest77("tests/GeneralStateTests/stRandom/randomStatetest77.json");
        // fn random_statetest135("tests/GeneralStateTests/stRandom/randomStatetest135.json");
        // fn random_statetest359("tests/GeneralStateTests/stRandom/randomStatetest359.json");
        // fn random_statetest20("tests/GeneralStateTests/stRandom/randomStatetest20.json");
        // fn random_statetest36("tests/GeneralStateTests/stRandom/randomStatetest36.json");
        // fn random_statetest174("tests/GeneralStateTests/stRandom/randomStatetest174.json");
        // fn random_statetest318("tests/GeneralStateTests/stRandom/randomStatetest318.json");
        // fn random_statetest60("tests/GeneralStateTests/stRandom/randomStatetest60.json");
        // fn random_statetest175("tests/GeneralStateTests/stRandom/randomStatetest175.json");
        // fn random_statetest37("tests/GeneralStateTests/stRandom/randomStatetest37.json");
        // fn random_statetest122("tests/GeneralStateTests/stRandom/randomStatetest122.json");
        // fn random_statetest358("tests/GeneralStateTests/stRandom/randomStatetest358.json");
        // fn random_statetest134("tests/GeneralStateTests/stRandom/randomStatetest134.json");
        // fn random_statetest163("tests/GeneralStateTests/stRandom/randomStatetest163.json");
        // fn random_statetest231("tests/GeneralStateTests/stRandom/randomStatetest231.json");
        // fn random_statetest118("tests/GeneralStateTests/stRandom/randomStatetest118.json");
        // fn random_statetest323("tests/GeneralStateTests/stRandom/randomStatetest323.json");
        // fn random_statetest266("tests/GeneralStateTests/stRandom/randomStatetest266.json");
        // fn random_statetest335("tests/GeneralStateTests/stRandom/randomStatetest335.json");
        // fn random_statetest270("tests/GeneralStateTests/stRandom/randomStatetest270.json");
        fn random_statetest159("tests/GeneralStateTests/stRandom/randomStatetest159.json"); // FAIL
        // fn random_statetest362("tests/GeneralStateTests/stRandom/randomStatetest362.json");
        // fn random_statetest1("tests/GeneralStateTests/stRandom/randomStatetest1.json");
        // fn random_statetest227("tests/GeneralStateTests/stRandom/randomStatetest227.json");
        // fn random_statetest196("tests/GeneralStateTests/stRandom/randomStatetest196.json");
        // fn random_statetest179("tests/GeneralStateTests/stRandom/randomStatetest179.json");
        // fn random_statetest250("tests/GeneralStateTests/stRandom/randomStatetest250.json");
        // fn random_statetest83("tests/GeneralStateTests/stRandom/randomStatetest83.json");
        // fn random_statetest315("tests/GeneralStateTests/stRandom/randomStatetest315.json");
        // fn random_statetest207("tests/GeneralStateTests/stRandom/randomStatetest207.json");
        // fn random_statetest342("tests/GeneralStateTests/stRandom/randomStatetest342.json");
        // fn random_statetest138("tests/GeneralStateTests/stRandom/randomStatetest138.json");
        // fn random_statetest211("tests/GeneralStateTests/stRandom/randomStatetest211.json");
        // fn random_statetest354("tests/GeneralStateTests/stRandom/randomStatetest354.json");
        // fn random_statetest180("tests/GeneralStateTests/stRandom/randomStatetest180.json");
        // fn random_statetest95("tests/GeneralStateTests/stRandom/randomStatetest95.json");
        // fn random_statetest246("tests/GeneralStateTests/stRandom/randomStatetest246.json");
        // fn random_statetest303("tests/GeneralStateTests/stRandom/randomStatetest303.json");
        // fn random_statetest114("tests/GeneralStateTests/stRandom/randomStatetest114.json");
        // fn random_statetest378("tests/GeneralStateTests/stRandom/randomStatetest378.json");
        // fn random_statetest143("tests/GeneralStateTests/stRandom/randomStatetest143.json");
        // fn random_statetest285("tests/GeneralStateTests/stRandom/randomStatetest285.json");
        // fn random_statetest155("tests/GeneralStateTests/stRandom/randomStatetest155.json");
        // fn random_statetest339("tests/GeneralStateTests/stRandom/randomStatetest339.json");
        // fn random_statetest293("tests/GeneralStateTests/stRandom/randomStatetest293.json");
        // fn random_statetest102("tests/GeneralStateTests/stRandom/randomStatetest102.json");
        // fn random_statetest17("tests/GeneralStateTests/stRandom/randomStatetest17.json");
        // fn random_statetest381("tests/GeneralStateTests/stRandom/randomStatetest381.json");
        // fn random_statetest6("tests/GeneralStateTests/stRandom/randomStatetest6.json");
        // fn random_statetest365("tests/GeneralStateTests/stRandom/randomStatetest365.json");
        // fn random_statetest220("tests/GeneralStateTests/stRandom/randomStatetest220.json");
        // fn random_statetest298("tests/GeneralStateTests/stRandom/randomStatetest298.json");
        // fn random_statetest332("tests/GeneralStateTests/stRandom/randomStatetest332.json");
        // fn random_statetest148("tests/GeneralStateTests/stRandom/randomStatetest148.json");
        // fn random_statetest261("tests/GeneralStateTests/stRandom/randomStatetest261.json");
        // fn random_statetest236("tests/GeneralStateTests/stRandom/randomStatetest236.json");
        // fn random_statetest164("tests/GeneralStateTests/stRandom/randomStatetest164.json");
        // fn random_statetest308("tests/GeneralStateTests/stRandom/randomStatetest308.json");
        // fn random_statetest133("tests/GeneralStateTests/stRandom/randomStatetest133.json");
        // fn random_statetest26("tests/GeneralStateTests/stRandom/randomStatetest26.json");
        // fn random_statetest125("tests/GeneralStateTests/stRandom/randomStatetest125.json");
        // fn random_statetest30("tests/GeneralStateTests/stRandom/randomStatetest30.json");
        // fn random_statetest349("tests/GeneralStateTests/stRandom/randomStatetest349.json");
        // fn random_statetest172("tests/GeneralStateTests/stRandom/randomStatetest172.json");
        // fn random_statetest88("tests/GeneralStateTests/stRandom/randomStatetest88.json");
        // fn random_statetest67("tests/GeneralStateTests/stRandom/randomStatetest67.json");
        // fn random_statetest10("tests/GeneralStateTests/stRandom/randomStatetest10.json");
        // fn random_statetest369("tests/GeneralStateTests/stRandom/randomStatetest369.json");
        // fn random_statetest105("tests/GeneralStateTests/stRandom/randomStatetest105.json");
        // fn random_statetest47("tests/GeneralStateTests/stRandom/randomStatetest47.json");
        // fn random_statetest294("tests/GeneralStateTests/stRandom/randomStatetest294.json");
        // fn random_statetest282("tests/GeneralStateTests/stRandom/randomStatetest282.json");
        // fn random_statetest51("tests/GeneralStateTests/stRandom/randomStatetest51.json");
        // fn random_statetest144("tests/GeneralStateTests/stRandom/randomStatetest144.json");
        // fn random_statetest92("tests/GeneralStateTests/stRandom/randomStatetest92.json");
        // fn random_statetest304("tests/GeneralStateTests/stRandom/randomStatetest304.json");
        // fn random_statetest187("tests/GeneralStateTests/stRandom/randomStatetest187.json");
        // fn random_statetest216("tests/GeneralStateTests/stRandom/randomStatetest216.json");
        // fn random_statetest353("tests/GeneralStateTests/stRandom/randomStatetest353.json");
        // fn random_statetest200("tests/GeneralStateTests/stRandom/randomStatetest200.json");
        // fn random_statetest345("tests/GeneralStateTests/stRandom/randomStatetest345.json");
        // fn random_statetest129("tests/GeneralStateTests/stRandom/randomStatetest129.json");
        // fn random_statetest84("tests/GeneralStateTests/stRandom/randomStatetest84.json");
        // fn random_statetest257("tests/GeneralStateTests/stRandom/randomStatetest257.json");
        // fn random_statetest312("tests/GeneralStateTests/stRandom/randomStatetest312.json");
        // fn random_statetest191("tests/GeneralStateTests/stRandom/randomStatetest191.json");
        // fn random_statetest190("tests/GeneralStateTests/stRandom/randomStatetest190.json");
        // fn random_statetest85("tests/GeneralStateTests/stRandom/randomStatetest85.json");
        // fn random_statetest313("tests/GeneralStateTests/stRandom/randomStatetest313.json");
        // fn random_statetest201("tests/GeneralStateTests/stRandom/randomStatetest201.json");
        // fn random_statetest217("tests/GeneralStateTests/stRandom/randomStatetest217.json");
        // fn random_statetest352("tests/GeneralStateTests/stRandom/randomStatetest352.json");
        // fn random_statetest169("tests/GeneralStateTests/stRandom/randomStatetest169.json");
        // fn random_statetest305("tests/GeneralStateTests/stRandom/randomStatetest305.json");
        // fn random_statetest112("tests/GeneralStateTests/stRandom/randomStatetest112.json");
        // fn random_statetest145("tests/GeneralStateTests/stRandom/randomStatetest145.json");
        // fn random_statetest283("tests/GeneralStateTests/stRandom/randomStatetest283.json");
        // fn random_statetest329("tests/GeneralStateTests/stRandom/randomStatetest329.json");
        // fn random_statetest153("tests/GeneralStateTests/stRandom/randomStatetest153.json");
        // fn random_statetest295("tests/GeneralStateTests/stRandom/randomStatetest295.json");
        // fn random_statetest104("tests/GeneralStateTests/stRandom/randomStatetest104.json");
        // fn random_statetest11("tests/GeneralStateTests/stRandom/randomStatetest11.json");
        // fn random_statetest368("tests/GeneralStateTests/stRandom/randomStatetest368.json");
        // fn random_statetest89("tests/GeneralStateTests/stRandom/randomStatetest89.json");
        // fn random_statetest66("tests/GeneralStateTests/stRandom/randomStatetest66.json");
        // fn random_statetest173("tests/GeneralStateTests/stRandom/randomStatetest173.json");
        // fn random_statetest31("tests/GeneralStateTests/stRandom/randomStatetest31.json");
        // fn random_statetest348("tests/GeneralStateTests/stRandom/randomStatetest348.json");
        // fn random_statetest124("tests/GeneralStateTests/stRandom/randomStatetest124.json");
        // fn random_statetest27("tests/GeneralStateTests/stRandom/randomStatetest27.json");
        // fn random_statetest309("tests/GeneralStateTests/stRandom/randomStatetest309.json");
        // fn random_statetest372("tests/GeneralStateTests/stRandom/randomStatetest372.json");
        // fn random_statetest237("tests/GeneralStateTests/stRandom/randomStatetest237.json");
        // fn random_statetest325("tests/GeneralStateTests/stRandom/randomStatetest325.json");
        // fn random_statetest260("tests/GeneralStateTests/stRandom/randomStatetest260.json");
        // fn random_statetest149("tests/GeneralStateTests/stRandom/randomStatetest149.json");
        // fn random_statetest299("tests/GeneralStateTests/stRandom/randomStatetest299.json");
        // fn random_statetest333("tests/GeneralStateTests/stRandom/randomStatetest333.json");
        // fn random_statetest276("tests/GeneralStateTests/stRandom/randomStatetest276.json");
        // fn random_statetest364("tests/GeneralStateTests/stRandom/randomStatetest364.json");
        // fn random_statetest221("tests/GeneralStateTests/stRandom/randomStatetest221.json");
        // fn random_statetest108("tests/GeneralStateTests/stRandom/randomStatetest108.json");
        // fn random_statetest259("tests/GeneralStateTests/stRandom/randomStatetest259.json");
        // fn random_statetest24("tests/GeneralStateTests/stRandom/randomStatetest24.json");
        // fn random_statetest131("tests/GeneralStateTests/stRandom/randomStatetest131.json");
        // fn random_statetest73("tests/GeneralStateTests/stRandom/randomStatetest73.json");
        // fn random_statetest166("tests/GeneralStateTests/stRandom/randomStatetest166.json");
        // fn random_statetest189("tests/GeneralStateTests/stRandom/randomStatetest189.json");
        // fn random_statetest371("tests/GeneralStateTests/stRandom/randomStatetest371.json");
        // fn random_statetest263("tests/GeneralStateTests/stRandom/randomStatetest263.json");
        // fn random_statetest326("tests/GeneralStateTests/stRandom/randomStatetest326.json");
        // fn random_statetest275("tests/GeneralStateTests/stRandom/randomStatetest275.json");
        // fn random_statetest49("tests/GeneralStateTests/stRandom/randomStatetest49.json");
        // fn random_statetest222("tests/GeneralStateTests/stRandom/randomStatetest222.json");
        // fn random_statetest367("tests/GeneralStateTests/stRandom/randomStatetest367.json");
        // fn random_statetest4("tests/GeneralStateTests/stRandom/randomStatetest4.json");
        // fn random_statetest69("tests/GeneralStateTests/stRandom/randomStatetest69.json");
        // fn random_statetest310("tests/GeneralStateTests/stRandom/randomStatetest310.json");
        // fn random_statetest347("tests/GeneralStateTests/stRandom/randomStatetest347.json");
        // fn random_statetest202("tests/GeneralStateTests/stRandom/randomStatetest202.json");
        // fn random_statetest28("tests/GeneralStateTests/stRandom/randomStatetest28.json");
        // fn random_statetest351("tests/GeneralStateTests/stRandom/randomStatetest351.json");
        // fn random_statetest214("tests/GeneralStateTests/stRandom/randomStatetest214.json");
        // fn random_statetest185("tests/GeneralStateTests/stRandom/randomStatetest185.json");
        fn random_statetest306("tests/GeneralStateTests/stRandom/randomStatetest306.json"); // FAIL
        // fn random_statetest90("tests/GeneralStateTests/stRandom/randomStatetest90.json");
        // fn random_statetest243("tests/GeneralStateTests/stRandom/randomStatetest243.json");
        // fn random_statetest111("tests/GeneralStateTests/stRandom/randomStatetest111.json");
        // fn random_statetest238("tests/GeneralStateTests/stRandom/randomStatetest238.json");
        // fn random_statetest146("tests/GeneralStateTests/stRandom/randomStatetest146.json");
        // fn random_statetest53("tests/GeneralStateTests/stRandom/randomStatetest53.json"); // CRASH
        // fn random_statetest280("tests/GeneralStateTests/stRandom/randomStatetest280.json");
        fn random_statetest150("tests/GeneralStateTests/stRandom/randomStatetest150.json"); // FAIL
        // fn random_statetest279("tests/GeneralStateTests/stRandom/randomStatetest279.json");
        // fn random_statetest296("tests/GeneralStateTests/stRandom/randomStatetest296.json");
        // fn random_statetest45("tests/GeneralStateTests/stRandom/randomStatetest45.json");
        // fn random_statetest107("tests/GeneralStateTests/stRandom/randomStatetest107.json");
        // fn random_statetest384("tests/GeneralStateTests/stRandom/randomStatetest384.json");
        // fn random_statetest12("tests/GeneralStateTests/stRandom/randomStatetest12.json");
        // fn random_statetest9("tests/GeneralStateTests/stRandom/randomStatetest9.json");
        // fn random_statetest13("tests/GeneralStateTests/stRandom/randomStatetest13.json");
        // fn random_statetest106("tests/GeneralStateTests/stRandom/randomStatetest106.json");
        // fn random_statetest278("tests/GeneralStateTests/stRandom/randomStatetest278.json");
        // fn random_statetest297("tests/GeneralStateTests/stRandom/randomStatetest297.json");
        // fn random_statetest151("tests/GeneralStateTests/stRandom/randomStatetest151.json");
        // fn random_statetest281("tests/GeneralStateTests/stRandom/randomStatetest281.json");
        // fn random_statetest52("tests/GeneralStateTests/stRandom/randomStatetest52.json");
        // fn random_statetest147("tests/GeneralStateTests/stRandom/randomStatetest147.json");
        // fn random_statetest110("tests/GeneralStateTests/stRandom/randomStatetest110.json");
        fn random_statetest307("tests/GeneralStateTests/stRandom/randomStatetest307.json"); // FAIL
        // fn random_statetest242("tests/GeneralStateTests/stRandom/randomStatetest242.json");
        // fn random_statetest184("tests/GeneralStateTests/stRandom/randomStatetest184.json");
        // fn random_statetest29("tests/GeneralStateTests/stRandom/randomStatetest29.json");
        // fn random_statetest350("tests/GeneralStateTests/stRandom/randomStatetest350.json");
        // fn random_statetest215("tests/GeneralStateTests/stRandom/randomStatetest215.json");
        // fn random_statetest346("tests/GeneralStateTests/stRandom/randomStatetest346.json");
        // fn random_statetest311("tests/GeneralStateTests/stRandom/randomStatetest311.json");
        // fn random_statetest87("tests/GeneralStateTests/stRandom/randomStatetest87.json");
        // fn random_statetest254("tests/GeneralStateTests/stRandom/randomStatetest254.json");
        // fn random_statetest192("tests/GeneralStateTests/stRandom/randomStatetest192.json");
        // fn random_statetest366("tests/GeneralStateTests/stRandom/randomStatetest366.json");
        // fn random_statetest5("tests/GeneralStateTests/stRandom/randomStatetest5.json");
        // fn random_statetest274("tests/GeneralStateTests/stRandom/randomStatetest274.json");
        fn random_statetest48("tests/GeneralStateTests/stRandom/randomStatetest48.json"); // FAIL
        // fn random_statetest327("tests/GeneralStateTests/stRandom/randomStatetest327.json");
        // fn random_statetest370("tests/GeneralStateTests/stRandom/randomStatetest370.json");
        // fn random_statetest167("tests/GeneralStateTests/stRandom/randomStatetest167.json");
        // fn random_statetest188("tests/GeneralStateTests/stRandom/randomStatetest188.json");
        // fn random_statetest72("tests/GeneralStateTests/stRandom/randomStatetest72.json");
        // fn random_statetest130("tests/GeneralStateTests/stRandom/randomStatetest130.json");
        // fn random_statetest25("tests/GeneralStateTests/stRandom/randomStatetest25.json");
        // fn random_statetest219("tests/GeneralStateTests/stRandom/randomStatetest219.json");
        // fn random_statetest126("tests/GeneralStateTests/stRandom/randomStatetest126.json");
        // fn random_statetest33("tests/GeneralStateTests/stRandom/randomStatetest33.json");
        // fn random_statetest171("tests/GeneralStateTests/stRandom/randomStatetest171.json");
        // fn random_statetest64("tests/GeneralStateTests/stRandom/randomStatetest64.json");
        // fn random_statetest195("tests/GeneralStateTests/stRandom/randomStatetest195.json");
        // fn random_statetest316("tests/GeneralStateTests/stRandom/randomStatetest316.json");
        // fn random_statetest80("tests/GeneralStateTests/stRandom/randomStatetest80.json");
        // fn random_statetest341("tests/GeneralStateTests/stRandom/randomStatetest341.json");
        // fn random_statetest204("tests/GeneralStateTests/stRandom/randomStatetest204.json");
        // fn random_statetest357("tests/GeneralStateTests/stRandom/randomStatetest357.json");
        // fn random_statetest212("tests/GeneralStateTests/stRandom/randomStatetest212.json");
        // fn random_statetest183("tests/GeneralStateTests/stRandom/randomStatetest183.json");
        // fn random_statetest300("tests/GeneralStateTests/stRandom/randomStatetest300.json");
        // fn random_statetest96("tests/GeneralStateTests/stRandom/randomStatetest96.json");
        // fn random_statetest245("tests/GeneralStateTests/stRandom/randomStatetest245.json");
        // fn random_statetest117("tests/GeneralStateTests/stRandom/randomStatetest117.json");
        // fn random_statetest269("tests/GeneralStateTests/stRandom/randomStatetest269.json");
        // fn random_statetest55("tests/GeneralStateTests/stRandom/randomStatetest55.json");
        // fn random_statetest286("tests/GeneralStateTests/stRandom/randomStatetest286.json");
        // fn random_statetest156("tests/GeneralStateTests/stRandom/randomStatetest156.json");
        // fn random_statetest290("tests/GeneralStateTests/stRandom/randomStatetest290.json");
        // fn random_statetest43("tests/GeneralStateTests/stRandom/randomStatetest43.json");
        // fn random_statetest382("tests/GeneralStateTests/stRandom/randomStatetest382.json");
        // fn random_statetest228("tests/GeneralStateTests/stRandom/randomStatetest228.json");
        // fn random_statetest14("tests/GeneralStateTests/stRandom/randomStatetest14.json");
        // fn random_statetest63("tests/GeneralStateTests/stRandom/randomStatetest63.json");
        // fn random_statetest176("tests/GeneralStateTests/stRandom/randomStatetest176.json");
        // fn random_statetest199("tests/GeneralStateTests/stRandom/randomStatetest199.json");
        // fn random_statetest208("tests/GeneralStateTests/stRandom/randomStatetest208.json");
        // fn random_statetest121("tests/GeneralStateTests/stRandom/randomStatetest121.json");
        // fn random_statetest22("tests/GeneralStateTests/stRandom/randomStatetest22.json");
        // fn random_statetest137("tests/GeneralStateTests/stRandom/randomStatetest137.json");
        // fn random_statetest75("tests/GeneralStateTests/stRandom/randomStatetest75.json");
        // fn random_statetest249("tests/GeneralStateTests/stRandom/randomStatetest249.json");
        // fn random_statetest232("tests/GeneralStateTests/stRandom/randomStatetest232.json");
        // fn random_statetest265("tests/GeneralStateTests/stRandom/randomStatetest265.json");
        // fn random_statetest320("tests/GeneralStateTests/stRandom/randomStatetest320.json");
        // fn random_statetest59("tests/GeneralStateTests/stRandom/randomStatetest59.json");
        // fn random_statetest273("tests/GeneralStateTests/stRandom/randomStatetest273.json");
        // fn random_statetest336("tests/GeneralStateTests/stRandom/randomStatetest336.json");
        // fn random_statetest2("tests/GeneralStateTests/stRandom/randomStatetest2.json");
        // fn random_statetest361("tests/GeneralStateTests/stRandom/randomStatetest361.json");
        // fn random_statetest18("tests/GeneralStateTests/stRandom/randomStatetest18.json");
        // fn random_statetest225("tests/GeneralStateTests/stRandom/randomStatetest225.json");
        // fn random_statetest3("tests/GeneralStateTests/stRandom/randomStatetest3.json");
        // fn random_statetest360("tests/GeneralStateTests/stRandom/randomStatetest360.json");
        // fn random_statetest19("tests/GeneralStateTests/stRandom/randomStatetest19.json");
        // fn random_statetest337("tests/GeneralStateTests/stRandom/randomStatetest337.json");
        // fn random_statetest264("tests/GeneralStateTests/stRandom/randomStatetest264.json");
        // fn random_statetest321("tests/GeneralStateTests/stRandom/randomStatetest321.json");
        // fn random_statetest58("tests/GeneralStateTests/stRandom/randomStatetest58.json");
        // fn random_statetest233("tests/GeneralStateTests/stRandom/randomStatetest233.json");
        // fn random_statetest376("tests/GeneralStateTests/stRandom/randomStatetest376.json");
        // fn random_statetest161("tests/GeneralStateTests/stRandom/randomStatetest161.json");
        // fn random_statetest74("tests/GeneralStateTests/stRandom/randomStatetest74.json");
        // fn random_statetest248("tests/GeneralStateTests/stRandom/randomStatetest248.json");
        // fn random_statetest23("tests/GeneralStateTests/stRandom/randomStatetest23.json");
        // fn random_statetest120("tests/GeneralStateTests/stRandom/randomStatetest120.json");
        // fn random_statetest209("tests/GeneralStateTests/stRandom/randomStatetest209.json");
        // fn random_statetest177("tests/GeneralStateTests/stRandom/randomStatetest177.json");
        // fn random_statetest198("tests/GeneralStateTests/stRandom/randomStatetest198.json");
        // fn random_statetest62("tests/GeneralStateTests/stRandom/randomStatetest62.json");
        // fn random_statetest383("tests/GeneralStateTests/stRandom/randomStatetest383.json");
        // fn random_statetest15("tests/GeneralStateTests/stRandom/randomStatetest15.json");
        // fn random_statetest100("tests/GeneralStateTests/stRandom/randomStatetest100.json");
        // fn random_statetest42("tests/GeneralStateTests/stRandom/randomStatetest42.json");
        // fn random_statetest291("tests/GeneralStateTests/stRandom/randomStatetest291.json");
        // fn random_statetest157("tests/GeneralStateTests/stRandom/randomStatetest157.json");
        // fn random_statetest268("tests/GeneralStateTests/stRandom/randomStatetest268.json");
        // fn random_statetest287("tests/GeneralStateTests/stRandom/randomStatetest287.json");
        // fn random_statetest54("tests/GeneralStateTests/stRandom/randomStatetest54.json");
        // fn random_statetest116("tests/GeneralStateTests/stRandom/randomStatetest116.json");
        // fn random_statetest301("tests/GeneralStateTests/stRandom/randomStatetest301.json");
        // fn random_statetest78("tests/GeneralStateTests/stRandom/randomStatetest78.json");
        // fn random_statetest244("tests/GeneralStateTests/stRandom/randomStatetest244.json");
        // fn random_statetest97("tests/GeneralStateTests/stRandom/randomStatetest97.json");
        // fn random_statetest356("tests/GeneralStateTests/stRandom/randomStatetest356.json");
        // fn random_statetest340("tests/GeneralStateTests/stRandom/randomStatetest340.json");
        // fn random_statetest39("tests/GeneralStateTests/stRandom/randomStatetest39.json");
        fn random_statetest205("tests/GeneralStateTests/stRandom/randomStatetest205.json"); // FAIL
        // fn random_statetest81("tests/GeneralStateTests/stRandom/randomStatetest81.json");
        // fn random_statetest252("tests/GeneralStateTests/stRandom/randomStatetest252.json");
        // fn random_statetest194("tests/GeneralStateTests/stRandom/randomStatetest194.json");


        // --- ALL PASS ---
        // fn returndatacopy_following_too_big_transfer("tests/GeneralStateTests/stReturnDataTest/returndatacopy_following_too_big_transfer.json");
        // fn returndatasize_bug("tests/GeneralStateTests/stReturnDataTest/returndatasize_bug.json");
        // fn returndatasize_initial_zero_read("tests/GeneralStateTests/stReturnDataTest/returndatasize_initial_zero_read.json");
        // fn returndatasize_following_successful_create("tests/GeneralStateTests/stReturnDataTest/returndatasize_following_successful_create.json");
        // fn returndatacopy_following_revert_in_create("tests/GeneralStateTests/stReturnDataTest/returndatacopy_following_revert_in_create.json");
        // fn returndatacopy_following_failing_call("tests/GeneralStateTests/stReturnDataTest/returndatacopy_following_failing_call.json");
        // fn returndatacopy_following_revert("tests/GeneralStateTests/stReturnDataTest/returndatacopy_following_revert.json");
        // fn subcall_return_more_then_expected("tests/GeneralStateTests/stReturnDataTest/subcallReturnMoreThenExpected.json");
        // fn returndatacopy_after_failing_callcode("tests/GeneralStateTests/stReturnDataTest/returndatacopy_after_failing_callcode.json");
        // fn returndatacopy_after_failing_create("tests/GeneralStateTests/stReturnDataTest/returndatacopy_afterFailing_create.json");
        // fn returndatacopy_following_successful_create("tests/GeneralStateTests/stReturnDataTest/returndatacopy_following_successful_create.json");
        // fn returndatacopy_after_failing_staticcall("tests/GeneralStateTests/stReturnDataTest/returndatacopy_after_failing_staticcall.json");
        // fn returndatasize_after_failing_delegatecall("tests/GeneralStateTests/stReturnDataTest/returndatasize_after_failing_delegatecall.json");
        // fn create_callprecompile_returndatasize("tests/GeneralStateTests/stReturnDataTest/create_callprecompile_returndatasize.json");
        // fn returndatacopy_overrun("tests/GeneralStateTests/stReturnDataTest/returndatacopy_overrun.json");
        // fn call_then_call_value_fail_then_returndatasize("tests/GeneralStateTests/stReturnDataTest/call_then_call_value_fail_then_returndatasize.json");
        // fn returndatasize_after_failing_staticcall("tests/GeneralStateTests/stReturnDataTest/returndatasize_after_failing_staticcall.json");
        // fn call_then_create_successful_then_returndatasize("tests/GeneralStateTests/stReturnDataTest/call_then_create_successful_then_returndatasize.json");
        // fn returndatasize_after_successful_delegatecall("tests/GeneralStateTests/stReturnDataTest/returndatasize_after_successful_delegatecall.json");
        // fn returndatacopy_following_create("tests/GeneralStateTests/stReturnDataTest/returndatacopy_following_create.json");
        // fn revert_ret_data_size("tests/GeneralStateTests/stReturnDataTest/revertRetDataSize.json");
        // fn too_long_return_data_copy("tests/GeneralStateTests/stReturnDataTest/tooLongReturnDataCopy.json");
        // fn returndatacopy_initial_big_sum("tests/GeneralStateTests/stReturnDataTest/returndatacopy_initial_big_sum.json");
        // fn call_ecrec_success_empty_then_returndatasize("tests/GeneralStateTests/stReturnDataTest/call_ecrec_success_empty_then_returndatasize.json");
        // fn returndatacopy_initial("tests/GeneralStateTests/stReturnDataTest/returndatacopy_initial.json");
        // fn returndatacopy_following_call("tests/GeneralStateTests/stReturnDataTest/returndatacopy_following_call.json");
        // fn returndatacopy_after_successful_callcode("tests/GeneralStateTests/stReturnDataTest/returndatacopy_after_successful_callcode.json");
        // fn returndatasize_after_oog_after_deeper("tests/GeneralStateTests/stReturnDataTest/returndatasize_after_oog_after_deeper.json");
        // fn modexp_modsize0_returndatasize("tests/GeneralStateTests/stReturnDataTest/modexp_modsize0_returndatasize.json");
        // fn returndatacopy_initial_256("tests/GeneralStateTests/stReturnDataTest/returndatacopy_initial_256.json");
        // fn returndatacopy_after_successful_staticcall("tests/GeneralStateTests/stReturnDataTest/returndatacopy_after_successful_staticcall.json");
        // fn returndatacopy_0_0_following_successful_create("tests/GeneralStateTests/stReturnDataTest/returndatacopy_0_0_following_successful_create.json");
        // fn clear_return_buffer("tests/GeneralStateTests/stReturnDataTest/clearReturnBuffer.json");
        // fn call_outsize_then_create_successful_then_returndatasize("tests/GeneralStateTests/stReturnDataTest/call_outsize_then_create_successful_then_returndatasize.json");
        // fn returndatacopy_after_revert_in_staticcall("tests/GeneralStateTests/stReturnDataTest/returndatacopy_after_revert_in_staticcall.json");
        // fn returndatasize_after_successful_callcode("tests/GeneralStateTests/stReturnDataTest/returndatasize_after_successful_callcode.json");
        // fn returndatacopy_after_failing_delegatecall("tests/GeneralStateTests/stReturnDataTest/returndatacopy_after_failing_delegatecall.json");
        // fn returndatasize_after_successful_staticcall("tests/GeneralStateTests/stReturnDataTest/returndatasize_after_successful_staticcall.json");
        // fn returndatasize_initial("tests/GeneralStateTests/stReturnDataTest/returndatasize_initial.json");
        // fn returndatacopy_after_successful_delegatecall("tests/GeneralStateTests/stReturnDataTest/returndatacopy_after_successful_delegatecall.json");
        // fn returndatasize_after_failing_callcode("tests/GeneralStateTests/stReturnDataTest/returndatasize_after_failing_callcode.json");
    }
}
