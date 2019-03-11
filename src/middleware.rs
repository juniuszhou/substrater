use sapper::{
    Request,
    Error as SapperError
};
use sapper_std::*;
use crate::AppUser;

pub fn permission_need_login(req: &mut Request) -> Result<(), SapperError> {
    let (path, _) = req.uri();
    if path.starts_with("/s/") || path.starts_with("/p/")
    {
        match ext_type!(req, AppUser) {
            Some(ref _user) => {
                // pass, nothing need to do here
                return Ok(());
            },
            None => {
                return res_400!("No permissions: need login.".to_string());
            }
        }
    }
    else {
        Ok(())
    }
}

pub fn permission_need_be_admin(req: &mut Request) -> Result<(), SapperError> {
    let (path, _) = req.uri();
    if path.starts_with("/s/") || path.starts_with("/p/")
    {
        match ext_type!(req, AppUser) {
            Some(user) => {
                if user.role >= 9 {
                    // pass, nothing need to do here
                    return Ok(());

                }
                else {
                    return res_400!("No permissions: need be admin.".to_string());
                }
            },
            None => {
                return res_400!("No permissions: need login.".to_string());
            }
        }
    }
    else {
        Ok(())
    }
}

pub fn is_admin(req: &mut Request) -> bool {
    let user = ext_type!(req, AppUser).unwrap();
    if user.role >= 9 {
        true
    }
    else {
        false
    }
}

