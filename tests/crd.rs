mod helper;

use chess::chess::crd::Crd;
    

#[test]
fn check_create_crd() {

    assert!( helper::check_option(&Crd::create(1,2)) );
    assert!( !helper::check_option(&Crd::create(100,2)) );
    assert!( !helper::check_option(&Crd::create(1,8)) );
    assert!( helper::check_option(&Crd::create(1,7)) );
    assert!( !helper::check_option(&Crd::create(8,6)) );
}
