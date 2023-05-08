// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(feature = "v2_70")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
use crate::InetAddress;
use crate::{SocketConnectable, TlsCertificateFlags};
#[cfg(feature = "v2_70")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
use glib::signal::{connect_raw, SignalHandlerId};
use glib::{prelude::*, translate::*};
#[cfg(feature = "v2_70")]
#[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
use std::{boxed::Box as Box_, mem::transmute};
use std::{fmt, ptr};

glib::wrapper! {
    #[doc(alias = "GTlsCertificate")]
    pub struct TlsCertificate(Object<ffi::GTlsCertificate, ffi::GTlsCertificateClass>);

    match fn {
        type_ => || ffi::g_tls_certificate_get_type(),
    }
}

impl TlsCertificate {
    pub const NONE: Option<&'static TlsCertificate> = None;

    #[doc(alias = "g_tls_certificate_new_from_file")]
    #[doc(alias = "new_from_file")]
    pub fn from_file(file: impl AsRef<std::path::Path>) -> Result<TlsCertificate, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_tls_certificate_new_from_file(file.as_ref().to_glib_none().0, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(feature = "v2_72")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_72")))]
    #[doc(alias = "g_tls_certificate_new_from_file_with_password")]
    #[doc(alias = "new_from_file_with_password")]
    pub fn from_file_with_password(
        file: impl AsRef<std::path::Path>,
        password: &str,
    ) -> Result<TlsCertificate, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_certificate_new_from_file_with_password(
                file.as_ref().to_glib_none().0,
                password.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_tls_certificate_new_from_files")]
    #[doc(alias = "new_from_files")]
    pub fn from_files(
        cert_file: impl AsRef<std::path::Path>,
        key_file: impl AsRef<std::path::Path>,
    ) -> Result<TlsCertificate, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_certificate_new_from_files(
                cert_file.as_ref().to_glib_none().0,
                key_file.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_tls_certificate_new_from_pem")]
    #[doc(alias = "new_from_pem")]
    pub fn from_pem(data: &str) -> Result<TlsCertificate, glib::Error> {
        let length = data.len() as _;
        unsafe {
            let mut error = ptr::null_mut();
            let ret =
                ffi::g_tls_certificate_new_from_pem(data.to_glib_none().0, length, &mut error);
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(feature = "v2_68")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_68")))]
    #[doc(alias = "g_tls_certificate_new_from_pkcs11_uris")]
    #[doc(alias = "new_from_pkcs11_uris")]
    pub fn from_pkcs11_uris(
        pkcs11_uri: &str,
        private_key_pkcs11_uri: Option<&str>,
    ) -> Result<TlsCertificate, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_certificate_new_from_pkcs11_uris(
                pkcs11_uri.to_glib_none().0,
                private_key_pkcs11_uri.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[cfg(feature = "v2_72")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_72")))]
    #[doc(alias = "g_tls_certificate_new_from_pkcs12")]
    #[doc(alias = "new_from_pkcs12")]
    pub fn from_pkcs12(data: &[u8], password: Option<&str>) -> Result<TlsCertificate, glib::Error> {
        let length = data.len() as _;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_certificate_new_from_pkcs12(
                data.to_glib_none().0,
                length,
                password.to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    #[doc(alias = "g_tls_certificate_list_new_from_file")]
    pub fn list_new_from_file(
        file: impl AsRef<std::path::Path>,
    ) -> Result<Vec<TlsCertificate>, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_certificate_list_new_from_file(
                file.as_ref().to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(FromGlibPtrContainer::from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }
}

pub trait TlsCertificateExt: IsA<TlsCertificate> + 'static {
    #[cfg(feature = "v2_70")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
    #[doc(alias = "g_tls_certificate_get_dns_names")]
    #[doc(alias = "get_dns_names")]
    fn dns_names(&self) -> Vec<glib::Bytes> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::g_tls_certificate_get_dns_names(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v2_70")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
    #[doc(alias = "g_tls_certificate_get_ip_addresses")]
    #[doc(alias = "get_ip_addresses")]
    fn ip_addresses(&self) -> Vec<InetAddress> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::g_tls_certificate_get_ip_addresses(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_tls_certificate_get_issuer")]
    #[doc(alias = "get_issuer")]
    #[must_use]
    fn issuer(&self) -> Option<TlsCertificate> {
        unsafe {
            from_glib_none(ffi::g_tls_certificate_get_issuer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v2_70")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
    #[doc(alias = "g_tls_certificate_get_issuer_name")]
    #[doc(alias = "get_issuer_name")]
    fn issuer_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_tls_certificate_get_issuer_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v2_70")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
    #[doc(alias = "g_tls_certificate_get_not_valid_after")]
    #[doc(alias = "get_not_valid_after")]
    fn not_valid_after(&self) -> Option<glib::DateTime> {
        unsafe {
            from_glib_full(ffi::g_tls_certificate_get_not_valid_after(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v2_70")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
    #[doc(alias = "g_tls_certificate_get_not_valid_before")]
    #[doc(alias = "get_not_valid_before")]
    fn not_valid_before(&self) -> Option<glib::DateTime> {
        unsafe {
            from_glib_full(ffi::g_tls_certificate_get_not_valid_before(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[cfg(feature = "v2_70")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
    #[doc(alias = "g_tls_certificate_get_subject_name")]
    #[doc(alias = "get_subject_name")]
    fn subject_name(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_full(ffi::g_tls_certificate_get_subject_name(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_tls_certificate_is_same")]
    fn is_same(&self, cert_two: &impl IsA<TlsCertificate>) -> bool {
        unsafe {
            from_glib(ffi::g_tls_certificate_is_same(
                self.as_ref().to_glib_none().0,
                cert_two.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_tls_certificate_verify")]
    fn verify(
        &self,
        identity: Option<&impl IsA<SocketConnectable>>,
        trusted_ca: Option<&impl IsA<TlsCertificate>>,
    ) -> TlsCertificateFlags {
        unsafe {
            from_glib(ffi::g_tls_certificate_verify(
                self.as_ref().to_glib_none().0,
                identity.map(|p| p.as_ref()).to_glib_none().0,
                trusted_ca.map(|p| p.as_ref()).to_glib_none().0,
            ))
        }
    }

    fn certificate(&self) -> Option<glib::ByteArray> {
        glib::ObjectExt::property(self.as_ref(), "certificate")
    }

    #[doc(alias = "certificate-pem")]
    fn certificate_pem(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "certificate-pem")
    }

    #[cfg(feature = "v2_68")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_68")))]
    #[doc(alias = "pkcs11-uri")]
    fn pkcs11_uri(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "pkcs11-uri")
    }

    #[doc(alias = "private-key")]
    fn private_key(&self) -> Option<glib::ByteArray> {
        glib::ObjectExt::property(self.as_ref(), "private-key")
    }

    #[doc(alias = "private-key-pem")]
    fn private_key_pem(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "private-key-pem")
    }

    #[cfg(feature = "v2_68")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_68")))]
    #[doc(alias = "private-key-pkcs11-uri")]
    fn private_key_pkcs11_uri(&self) -> Option<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "private-key-pkcs11-uri")
    }

    #[cfg(feature = "v2_70")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
    #[doc(alias = "dns-names")]
    fn connect_dns_names_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_dns_names_trampoline<
            P: IsA<TlsCertificate>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsCertificate,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TlsCertificate::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::dns-names\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_dns_names_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v2_70")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
    #[doc(alias = "ip-addresses")]
    fn connect_ip_addresses_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ip_addresses_trampoline<
            P: IsA<TlsCertificate>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsCertificate,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TlsCertificate::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ip-addresses\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ip_addresses_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v2_70")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
    #[doc(alias = "issuer-name")]
    fn connect_issuer_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_issuer_name_trampoline<
            P: IsA<TlsCertificate>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsCertificate,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TlsCertificate::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::issuer-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_issuer_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v2_70")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
    #[doc(alias = "not-valid-after")]
    fn connect_not_valid_after_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_not_valid_after_trampoline<
            P: IsA<TlsCertificate>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsCertificate,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TlsCertificate::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::not-valid-after\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_not_valid_after_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v2_70")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
    #[doc(alias = "not-valid-before")]
    fn connect_not_valid_before_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_not_valid_before_trampoline<
            P: IsA<TlsCertificate>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsCertificate,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TlsCertificate::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::not-valid-before\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_not_valid_before_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(feature = "v2_70")]
    #[cfg_attr(docsrs, doc(cfg(feature = "v2_70")))]
    #[doc(alias = "subject-name")]
    fn connect_subject_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_subject_name_trampoline<
            P: IsA<TlsCertificate>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GTlsCertificate,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(TlsCertificate::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::subject-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_subject_name_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<TlsCertificate>> TlsCertificateExt for O {}

impl fmt::Display for TlsCertificate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TlsCertificate")
    }
}
