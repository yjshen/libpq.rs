/**
 * [Connection Status Functions](https://www.postgresql.org/docs/current/libpq-status.html)
 */
impl Connection {
    /**
     * Returns the database name of the connection.
     *
     * See [PQdb](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQDB).
     */
    pub fn db(&self) -> String {
        crate::ffi::to_string(unsafe { pq_sys::PQdb(self.into()) })
    }

    /**
     * Returns the user name of the connection.
     *
     * See [PQuser](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQUSER).
     */
    pub fn user(&self) -> String {
        crate::ffi::to_string(unsafe { pq_sys::PQuser(self.into()) })
    }

    /**
     * Returns the password of the connection.
     *
     * See [PQpass](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQPASS).
     */
    pub fn pass(&self) -> Option<String> {
        crate::ffi::to_option_string(unsafe { pq_sys::PQpass(self.into()) })
    }

    /**
     * Returns the server host name of the active connection.
     *
     * This can be a host name, an IP address, or a directory path if the connection is via Unix
     * socket. (The path case can be distinguished because it will always be an absolute path,
     * beginning with /.)
     *
     * See [PQhost](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQHOST).
     */
    pub fn host(&self) -> String {
        crate::ffi::to_string(unsafe { pq_sys::PQhost(self.into()) })
    }

    /**
     * Returns the server IP address of the active connection.
     *
     * This can be the address that a host name resolved to, or an IP address provided through the
     * hostaddr parameter.
     *
     * See [PQhostaddr](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQHOSTADDR).
     */
    #[cfg(feature = "v12")]
    pub fn hostaddr(&self) -> String {
        crate::ffi::to_string(unsafe { pq_sys::PQhostaddr(self.into()) })
    }

    /**
     * Returns the port of the active connection.
     *
     * See [PQport](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQPORT).
     */
    pub fn port(&self) -> String {
        crate::ffi::to_string(unsafe { pq_sys::PQport(self.into()) })
    }

    /**
     * Returns the debug TTY of the connection.
     *
     * See [PQtty](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQTTY).
     */
    #[deprecated(
        note = "the server no longer pays attention to the TTY setting, but the function remains for backward compatibility."
    )]
    pub fn tty(&self) -> Option<String> {
        crate::ffi::to_option_string(unsafe { pq_sys::PQtty(self.into()) })
    }

    /**
     * Returns the command-line options passed in the connection request.
     *
     * See [PQoptions](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQOPTIONS).
     */
    pub fn options(&self) -> Option<String> {
        crate::ffi::to_option_string(unsafe { pq_sys::PQoptions(self.into()) })
    }

    /**
     * Returns the status of the connection.
     *
     * See [PQstatus](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSTATUS).
     */
    pub fn status(&self) -> crate::connection::Status {
        unsafe { pq_sys::PQstatus(self.into()) }.into()
    }

    /**
     * Returns the current in-transaction status of the server.
     *
     * See [PQtransactionStatus](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQTRANSACTIONSTATUS).
     */
    pub fn transaction_status(&self) -> crate::transaction::Status {
        unsafe { pq_sys::PQtransactionStatus(self.into()) }.into()
    }

    /**
     * Looks up a current parameter setting of the server.
     *
     * See [PQparameterStatus](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQPARAMETERSTATUS).
     */
    pub fn parameter_status(&self, param: &str) -> String {
        let c_param = crate::ffi::to_cstr(param);

        crate::ffi::to_string(unsafe { pq_sys::PQparameterStatus(self.into(), c_param.as_ptr()) })
    }

    /**
     * Interrogates the frontend/backend protocol being used.
     *
     * See [PQprotocolVersion](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQPROTOCOLVERSION).
     */
    pub fn protocol_version(&self) -> i32 {
        unsafe { pq_sys::PQprotocolVersion(self.into()) }
    }

    /**
     * Returns an integer representing the server version.
     *
     * See [PQserverVersion](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSERVERVERSION).
     */
    pub fn server_version(&self) -> i32 {
        unsafe { pq_sys::PQserverVersion(self.into()) }
    }

    /**
     * Returns the error message most recently generated by an operation on the connection.
     *
     * See [PQerrorMessage](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQERRORMESSAGE).
     *
     * # Implemenatation Notes:
     * PQerrorMessage returns a localizable string, which depends on the locale settings and
     * is it not guaranteed to be UTF-8 encoded.
     *
     * Since rust strings are UTF-8 encoded, if the error message cannot be converted to UTF-8,
     * the returned string will not be the actual error message but
     * "PQerrorMessage internal error: the error message is not UTF-8".
     * You can get the actual error message, by changing your locale settings an UTF-8 compatible one.
     */
    pub fn error_message(&self) -> Option<&str> {
        let error = unsafe { pq_sys::PQerrorMessage(self.into()) };
        if error.is_null() {
            None
        } else {
            //SAFETY: the pointer is valid because we checked it above.
            match unsafe { std::ffi::CStr::from_ptr(error) }.to_str() {
                Ok(s) => Some(s),
                Err(_) => Some("PQerrorMessage internal error: the error message is not UTF-8"),
            }
        }
    }

    /**
     * Obtains the file descriptor number of the connection socket to the server.
     *
     * See [PQsocket](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSOCKET).
     */
    pub fn socket(&self) -> std::result::Result<i32, ()> {
        let socket = unsafe { pq_sys::PQsocket(self.into()) };

        if socket < 0 {
            Err(())
        } else {
            Ok(socket)
        }
    }

    /**
     * Returns the process ID (PID) of the backend process handling this connection.
     *
     * See [PQbackendPID](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQBACKENDPID).
     */
    pub fn backend_pid(&self) -> u32 {
        unsafe { pq_sys::PQbackendPID(self.into()) as u32 }
    }

    /**
     * Returns `true` if the connection authentication method required a password, but none was
     * available. Returns `false` if not.
     *
     * See [PQconnectionNeedsPassword](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQCONNECTIONNEEDSPASSWORD).
     */
    pub fn needs_password(&self) -> bool {
        unsafe { pq_sys::PQconnectionNeedsPassword(self.into()) == 1 }
    }

    /**
     * Returns `true` if the connection authentication method used a password. Returns `false` if
     * not.
     *
     * See [PQconnectionUsedPassword](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQCONNECTIONUSEDPASSWORD).
     */
    pub fn used_password(&self) -> bool {
        unsafe { pq_sys::PQconnectionUsedPassword(self.into()) == 1 }
    }

    /**
     * Returns `true` if the connection uses SSL, `false` if not.
     *
     * See [PQsslInUse](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSSLINUSE).
     */
    pub fn ssl_in_use(&self) -> bool {
        unsafe { pq_sys::PQsslInUse(self.into()) == 1 }
    }

    /**
     * Returns SSL-related information about the connection.
     *
     * See [PQsslAttribute](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSSLATTRIBUTE).
     */
    pub fn ssl_attribute(&self, attribute: crate::ssl::Attribute) -> Option<String> {
        let c_attribute = crate::ffi::to_cstr(&attribute.to_string());

        let raw = unsafe { pq_sys::PQsslAttribute(self.into(), c_attribute.as_ptr()) };

        if raw.is_null() {
            None
        } else {
            crate::ffi::to_option_string(raw)
        }
    }

    /**
     * Return an array of SSL attribute names available.
     *
     * See [PQsslAttributeNames](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSSLATTRIBUTENAMES).
     */
    pub fn ssl_attribute_names(&self) -> Vec<crate::ssl::Attribute> {
        let raw = unsafe { pq_sys::PQsslAttributeNames(self.into()) };

        crate::ffi::vec_from_nta(raw)
            .iter()
            .map(|x| x.into())
            .collect()
    }

    /**
     * Return a pointer to an SSL-implementation-specific object describing the connection.
     *
     * See [PQsslStruct](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQSSLSTRUCT).
     *
     * # Safety
     *
     * This function returns a `void*` pointer.
     */
    pub unsafe fn ssl_struct(&self, struct_name: &str) -> *const std::ffi::c_void {
        let c_struct_name = crate::ffi::to_cstr(struct_name);

        pq_sys::PQsslStruct(self.into(), c_struct_name.as_ptr())
    }

    /**
     * Returns the SSL structure used in the connection, or null if SSL is not in use.
     *
     * See [PQgetssl](https://www.postgresql.org/docs/current/libpq-status.html#LIBPQ-PQGETSSL).
     *
     * # Safety
     *
     * This function returns a `void*` pointer.
     */
    pub unsafe fn ssl(&self) -> *const std::ffi::c_void {
        pq_sys::PQgetssl(self.into())
    }
}
