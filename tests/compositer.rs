extern crate neko;

use neko::prelude::*;

use self::std::path::PathBuf;
use self::std::env;

#[test]
fn test_compositer_new() {
  assert!(Compositer::new().is_ok());
}

#[test]
fn test_compositer_command() {
    {
        let compositer: Compositer = Compositer::new().unwrap();

        env::set_var("HOME", "/tmp/arukana1");
        assert_eq!(compositer.get_git().ok(),
                Some(PathBuf::from("/tmp/arukana1/.neko/git"))
        );
    }
    {
        let compositer: Compositer = Compositer::new().unwrap();

        env::set_var("HOME", "/tmp/arukana2");
        assert_eq!(compositer.get_lib().ok(),
                Some(PathBuf::from("/tmp/arukana2/.neko/lib"))
        );
    }
    {
        let mut compositer: Compositer = Compositer::new().unwrap();

        env::set_var("HOME", "/tmp/arukana3");
        assert!(compositer.install(
            "https://github.com/Arukana/libnya.git"
        ).is_ok());
        assert!(compositer.mount(
            &"arukana@libnya", None
        ).is_ok());
        assert!(compositer.mount(
            &"arukana@libnya", None
        ).is_ok());
        assert!(compositer.update(
            "arukana@libnya"
        ).is_ok());
        assert!(compositer.install(
            "https://github.com/Arukana/libnya.git"
        ).is_err());
        assert!(compositer.unmount(
            "arukana@libnya"
        ).is_ok());
        assert!(compositer.unmount(
            "arukana@libnya"
        ).is_err());
        assert!(compositer.uninstall(
            &"arukana@libnya"
        ).is_ok());
        assert!(compositer.unmount(
            "arukana@libnya"
        ).is_err());
    }
    {
        let mut compositer: Compositer = Compositer::new().unwrap();

        env::set_var("HOME", "/tmp/arukana4");
        assert!(compositer.install(
            "https://github.com/Arukana/libnya.git"
        ).is_ok());
        assert!(compositer.uninstall(
            &"arukana@libnya"
        ).is_ok());
        assert!(compositer.uninstall(
            &"arukana@libnya"
        ).is_err());
    }
}
