mod suite;
use toybank::basic::{Accountant,Ledger};
use toybank::common::Policy;

struct TheFactory;

impl suite::Factory for TheFactory {
    fn open(_: Option<String>, _: Policy) -> suite::DynBank {
        panic!("open is not implemented for basic::Accountant using basic::Ledger");
    }
    fn new(_: Option<String>, policy: Policy) -> suite::DynBank {
        return suite::dyn_wrap(Accountant::with_policy(Ledger::default(), policy));
    }
}

#[test]
fn test() {
    suite::succeeded_with::<TheFactory>("tests/features/basic");
}