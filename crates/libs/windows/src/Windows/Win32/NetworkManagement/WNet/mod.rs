#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CONNECTDLGSTRUCTA {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpConnRes: *mut NETRESOURCEA,
    pub dwFlags: CONNECTDLGSTRUCT_FLAGS,
    pub dwDevNum: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CONNECTDLGSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONNECTDLGSTRUCTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CONNECTDLGSTRUCTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONNECTDLGSTRUCTA").field("cbStructure", &self.cbStructure).field("hwndOwner", &self.hwndOwner).field("lpConnRes", &self.lpConnRes).field("dwFlags", &self.dwFlags).field("dwDevNum", &self.dwDevNum).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CONNECTDLGSTRUCTA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CONNECTDLGSTRUCTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONNECTDLGSTRUCTA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CONNECTDLGSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CONNECTDLGSTRUCTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct CONNECTDLGSTRUCTW {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpConnRes: *mut NETRESOURCEW,
    pub dwFlags: CONNECTDLGSTRUCT_FLAGS,
    pub dwDevNum: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CONNECTDLGSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CONNECTDLGSTRUCTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for CONNECTDLGSTRUCTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CONNECTDLGSTRUCTW").field("cbStructure", &self.cbStructure).field("hwndOwner", &self.hwndOwner).field("lpConnRes", &self.lpConnRes).field("dwFlags", &self.dwFlags).field("dwDevNum", &self.dwDevNum).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for CONNECTDLGSTRUCTW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for CONNECTDLGSTRUCTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CONNECTDLGSTRUCTW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for CONNECTDLGSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for CONNECTDLGSTRUCTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONNECTDLGSTRUCT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNDLG_RO_PATH: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNDLG_CONN_POINT: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNDLG_USE_MRU: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNDLG_HIDE_BOX: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNDLG_PERSIST: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNDLG_NOT_PERSIST: CONNECTDLGSTRUCT_FLAGS = CONNECTDLGSTRUCT_FLAGS(32u32);
impl ::core::marker::Copy for CONNECTDLGSTRUCT_FLAGS {}
impl ::core::clone::Clone for CONNECTDLGSTRUCT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONNECTDLGSTRUCT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CONNECTDLGSTRUCT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for CONNECTDLGSTRUCT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONNECTDLGSTRUCT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for CONNECTDLGSTRUCT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for CONNECTDLGSTRUCT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for CONNECTDLGSTRUCT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for CONNECTDLGSTRUCT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for CONNECTDLGSTRUCT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_CRED_RESET: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_CURRENT_MEDIA: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_GLOBAL_MAPPING: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_LOCALDRIVE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_NEED_DRIVE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_REFCOUNT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_REQUIRE_INTEGRITY: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_REQUIRE_PRIVACY: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_RESERVED: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_WRITE_THROUGH_SEMANTICS: u32 = 65536u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DISCDLGSTRUCTA {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpLocalName: ::windows::core::PSTR,
    pub lpRemoteName: ::windows::core::PSTR,
    pub dwFlags: DISCDLGSTRUCT_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DISCDLGSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DISCDLGSTRUCTA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISCDLGSTRUCTA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISCDLGSTRUCTA").field("cbStructure", &self.cbStructure).field("hwndOwner", &self.hwndOwner).field("lpLocalName", &self.lpLocalName).field("lpRemoteName", &self.lpRemoteName).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISCDLGSTRUCTA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISCDLGSTRUCTA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISCDLGSTRUCTA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISCDLGSTRUCTA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISCDLGSTRUCTA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DISCDLGSTRUCTW {
    pub cbStructure: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub lpLocalName: ::windows::core::PWSTR,
    pub lpRemoteName: ::windows::core::PWSTR,
    pub dwFlags: DISCDLGSTRUCT_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DISCDLGSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DISCDLGSTRUCTW {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DISCDLGSTRUCTW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DISCDLGSTRUCTW").field("cbStructure", &self.cbStructure).field("hwndOwner", &self.hwndOwner).field("lpLocalName", &self.lpLocalName).field("lpRemoteName", &self.lpRemoteName).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DISCDLGSTRUCTW {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DISCDLGSTRUCTW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DISCDLGSTRUCTW>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DISCDLGSTRUCTW {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DISCDLGSTRUCTW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DISCDLGSTRUCT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const DISC_UPDATE_PROFILE: DISCDLGSTRUCT_FLAGS = DISCDLGSTRUCT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const DISC_NO_FORCE: DISCDLGSTRUCT_FLAGS = DISCDLGSTRUCT_FLAGS(64u32);
impl ::core::marker::Copy for DISCDLGSTRUCT_FLAGS {}
impl ::core::clone::Clone for DISCDLGSTRUCT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DISCDLGSTRUCT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for DISCDLGSTRUCT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for DISCDLGSTRUCT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DISCDLGSTRUCT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DISCDLGSTRUCT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DISCDLGSTRUCT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DISCDLGSTRUCT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DISCDLGSTRUCT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DISCDLGSTRUCT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn MultinetGetConnectionPerformanceA(lpnetresource: *const NETRESOURCEA, lpnetconnectinfostruct: *mut NETCONNECTINFOSTRUCT) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MultinetGetConnectionPerformanceA(lpnetresource: *const NETRESOURCEA, lpnetconnectinfostruct: *mut NETCONNECTINFOSTRUCT) -> u32;
    }
    MultinetGetConnectionPerformanceA(::core::mem::transmute(lpnetresource), ::core::mem::transmute(lpnetconnectinfostruct))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn MultinetGetConnectionPerformanceW(lpnetresource: *const NETRESOURCEW, lpnetconnectinfostruct: *mut NETCONNECTINFOSTRUCT) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn MultinetGetConnectionPerformanceW(lpnetresource: *const NETRESOURCEW, lpnetconnectinfostruct: *mut NETCONNECTINFOSTRUCT) -> u32;
    }
    MultinetGetConnectionPerformanceW(::core::mem::transmute(lpnetresource), ::core::mem::transmute(lpnetconnectinfostruct))
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub struct NETCONNECTINFOSTRUCT {
    pub cbStructure: u32,
    pub dwFlags: u32,
    pub dwSpeed: u32,
    pub dwDelay: u32,
    pub dwOptDataSize: u32,
}
impl ::core::marker::Copy for NETCONNECTINFOSTRUCT {}
impl ::core::clone::Clone for NETCONNECTINFOSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NETCONNECTINFOSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETCONNECTINFOSTRUCT").field("cbStructure", &self.cbStructure).field("dwFlags", &self.dwFlags).field("dwSpeed", &self.dwSpeed).field("dwDelay", &self.dwDelay).field("dwOptDataSize", &self.dwOptDataSize).finish()
    }
}
unsafe impl ::windows::core::Abi for NETCONNECTINFOSTRUCT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NETCONNECTINFOSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NETCONNECTINFOSTRUCT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NETCONNECTINFOSTRUCT {}
impl ::core::default::Default for NETCONNECTINFOSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NETINFOSTRUCT {
    pub cbStructure: u32,
    pub dwProviderVersion: u32,
    pub dwStatus: super::super::Foundation::WIN32_ERROR,
    pub dwCharacteristics: NETINFOSTRUCT_CHARACTERISTICS,
    pub dwHandle: usize,
    pub wNetType: u16,
    pub dwPrinters: u32,
    pub dwDrives: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NETINFOSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NETINFOSTRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NETINFOSTRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETINFOSTRUCT").field("cbStructure", &self.cbStructure).field("dwProviderVersion", &self.dwProviderVersion).field("dwStatus", &self.dwStatus).field("dwCharacteristics", &self.dwCharacteristics).field("dwHandle", &self.dwHandle).field("wNetType", &self.wNetType).field("dwPrinters", &self.dwPrinters).field("dwDrives", &self.dwDrives).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NETINFOSTRUCT {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NETINFOSTRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NETINFOSTRUCT>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NETINFOSTRUCT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NETINFOSTRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NETINFOSTRUCT_CHARACTERISTICS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const NETINFO_DLL16: NETINFOSTRUCT_CHARACTERISTICS = NETINFOSTRUCT_CHARACTERISTICS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const NETINFO_DISKRED: NETINFOSTRUCT_CHARACTERISTICS = NETINFOSTRUCT_CHARACTERISTICS(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const NETINFO_PRINTERRED: NETINFOSTRUCT_CHARACTERISTICS = NETINFOSTRUCT_CHARACTERISTICS(8u32);
impl ::core::marker::Copy for NETINFOSTRUCT_CHARACTERISTICS {}
impl ::core::clone::Clone for NETINFOSTRUCT_CHARACTERISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETINFOSTRUCT_CHARACTERISTICS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NETINFOSTRUCT_CHARACTERISTICS {
    type Abi = Self;
}
impl ::core::fmt::Debug for NETINFOSTRUCT_CHARACTERISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETINFOSTRUCT_CHARACTERISTICS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NETINFOSTRUCT_CHARACTERISTICS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NETINFOSTRUCT_CHARACTERISTICS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NETINFOSTRUCT_CHARACTERISTICS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NETINFOSTRUCT_CHARACTERISTICS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NETINFOSTRUCT_CHARACTERISTICS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const NETPROPERTY_PERSISTENT: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub struct NETRESOURCEA {
    pub dwScope: NET_RESOURCE_SCOPE,
    pub dwType: NET_RESOURCE_TYPE,
    pub dwDisplayType: u32,
    pub dwUsage: u32,
    pub lpLocalName: ::windows::core::PSTR,
    pub lpRemoteName: ::windows::core::PSTR,
    pub lpComment: ::windows::core::PSTR,
    pub lpProvider: ::windows::core::PSTR,
}
impl ::core::marker::Copy for NETRESOURCEA {}
impl ::core::clone::Clone for NETRESOURCEA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NETRESOURCEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETRESOURCEA").field("dwScope", &self.dwScope).field("dwType", &self.dwType).field("dwDisplayType", &self.dwDisplayType).field("dwUsage", &self.dwUsage).field("lpLocalName", &self.lpLocalName).field("lpRemoteName", &self.lpRemoteName).field("lpComment", &self.lpComment).field("lpProvider", &self.lpProvider).finish()
    }
}
unsafe impl ::windows::core::Abi for NETRESOURCEA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NETRESOURCEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NETRESOURCEA>()) == 0 }
    }
}
impl ::core::cmp::Eq for NETRESOURCEA {}
impl ::core::default::Default for NETRESOURCEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub struct NETRESOURCEW {
    pub dwScope: NET_RESOURCE_SCOPE,
    pub dwType: NET_RESOURCE_TYPE,
    pub dwDisplayType: u32,
    pub dwUsage: u32,
    pub lpLocalName: ::windows::core::PWSTR,
    pub lpRemoteName: ::windows::core::PWSTR,
    pub lpComment: ::windows::core::PWSTR,
    pub lpProvider: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for NETRESOURCEW {}
impl ::core::clone::Clone for NETRESOURCEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NETRESOURCEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETRESOURCEW").field("dwScope", &self.dwScope).field("dwType", &self.dwType).field("dwDisplayType", &self.dwDisplayType).field("dwUsage", &self.dwUsage).field("lpLocalName", &self.lpLocalName).field("lpRemoteName", &self.lpRemoteName).field("lpComment", &self.lpComment).field("lpProvider", &self.lpProvider).finish()
    }
}
unsafe impl ::windows::core::Abi for NETRESOURCEW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NETRESOURCEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NETRESOURCEW>()) == 0 }
    }
}
impl ::core::cmp::Eq for NETRESOURCEW {}
impl ::core::default::Default for NETRESOURCEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NETWORK_NAME_FORMAT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNFMT_MULTILINE: NETWORK_NAME_FORMAT_FLAGS = NETWORK_NAME_FORMAT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNFMT_ABBREVIATED: NETWORK_NAME_FORMAT_FLAGS = NETWORK_NAME_FORMAT_FLAGS(2u32);
impl ::core::marker::Copy for NETWORK_NAME_FORMAT_FLAGS {}
impl ::core::clone::Clone for NETWORK_NAME_FORMAT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NETWORK_NAME_FORMAT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NETWORK_NAME_FORMAT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for NETWORK_NAME_FORMAT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NETWORK_NAME_FORMAT_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_RESOURCE_SCOPE(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCE_CONNECTED: NET_RESOURCE_SCOPE = NET_RESOURCE_SCOPE(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCE_CONTEXT: NET_RESOURCE_SCOPE = NET_RESOURCE_SCOPE(5u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCE_GLOBALNET: NET_RESOURCE_SCOPE = NET_RESOURCE_SCOPE(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCE_REMEMBERED: NET_RESOURCE_SCOPE = NET_RESOURCE_SCOPE(3u32);
impl ::core::marker::Copy for NET_RESOURCE_SCOPE {}
impl ::core::clone::Clone for NET_RESOURCE_SCOPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_RESOURCE_SCOPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NET_RESOURCE_SCOPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_RESOURCE_SCOPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_RESOURCE_SCOPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_RESOURCE_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCETYPE_ANY: NET_RESOURCE_TYPE = NET_RESOURCE_TYPE(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCETYPE_DISK: NET_RESOURCE_TYPE = NET_RESOURCE_TYPE(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCETYPE_PRINT: NET_RESOURCE_TYPE = NET_RESOURCE_TYPE(2u32);
impl ::core::marker::Copy for NET_RESOURCE_TYPE {}
impl ::core::clone::Clone for NET_RESOURCE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_RESOURCE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NET_RESOURCE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_RESOURCE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_RESOURCE_TYPE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NET_RESOURCE_TYPE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NET_RESOURCE_TYPE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NET_RESOURCE_TYPE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NET_RESOURCE_TYPE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NET_RESOURCE_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NET_USE_CONNECT_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_INTERACTIVE: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_PROMPT: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_REDIRECT: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(128u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_UPDATE_PROFILE: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_COMMANDLINE: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(2048u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_CMD_SAVECRED: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(4096u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_TEMPORARY: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(4u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_DEFERRED: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(1024u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const CONNECT_UPDATE_RECENT: NET_USE_CONNECT_FLAGS = NET_USE_CONNECT_FLAGS(2u32);
impl ::core::marker::Copy for NET_USE_CONNECT_FLAGS {}
impl ::core::clone::Clone for NET_USE_CONNECT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NET_USE_CONNECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NET_USE_CONNECT_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for NET_USE_CONNECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NET_USE_CONNECT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for NET_USE_CONNECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for NET_USE_CONNECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for NET_USE_CONNECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for NET_USE_CONNECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for NET_USE_CONNECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NOTIFYADD {
    pub hwndOwner: super::super::Foundation::HWND,
    pub NetResource: NETRESOURCEA,
    pub dwAddFlags: NET_USE_CONNECT_FLAGS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NOTIFYADD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NOTIFYADD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NOTIFYADD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NOTIFYADD").field("hwndOwner", &self.hwndOwner).field("NetResource", &self.NetResource).field("dwAddFlags", &self.dwAddFlags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NOTIFYADD {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NOTIFYADD {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NOTIFYADD>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NOTIFYADD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NOTIFYADD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NOTIFYCANCEL {
    pub lpName: ::windows::core::PWSTR,
    pub lpProvider: ::windows::core::PWSTR,
    pub dwFlags: u32,
    pub fForce: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NOTIFYCANCEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NOTIFYCANCEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NOTIFYCANCEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NOTIFYCANCEL").field("lpName", &self.lpName).field("lpProvider", &self.lpProvider).field("dwFlags", &self.dwFlags).field("fForce", &self.fForce).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NOTIFYCANCEL {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NOTIFYCANCEL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NOTIFYCANCEL>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NOTIFYCANCEL {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NOTIFYCANCEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub struct NOTIFYINFO {
    pub dwNotifyStatus: u32,
    pub dwOperationStatus: u32,
    pub lpContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NOTIFYINFO {}
impl ::core::clone::Clone for NOTIFYINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NOTIFYINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NOTIFYINFO").field("dwNotifyStatus", &self.dwNotifyStatus).field("dwOperationStatus", &self.dwOperationStatus).field("lpContext", &self.lpContext).finish()
    }
}
unsafe impl ::windows::core::Abi for NOTIFYINFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NOTIFYINFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NOTIFYINFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for NOTIFYINFO {}
impl ::core::default::Default for NOTIFYINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const NOTIFY_POST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const NOTIFY_PRE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPAddConnection<'a, P0, P1>(lpnetresource: *const NETRESOURCEW, lppassword: P0, lpusername: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NPAddConnection(lpnetresource: *const NETRESOURCEW, lppassword: ::windows::core::PCWSTR, lpusername: ::windows::core::PCWSTR) -> u32;
    }
    NPAddConnection(::core::mem::transmute(lpnetresource), lppassword.into(), lpusername.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NPAddConnection3<'a, P0, P1, P2>(hwndowner: P0, lpnetresource: *const NETRESOURCEW, lppassword: P1, lpusername: P2, dwflags: NET_USE_CONNECT_FLAGS) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NPAddConnection3(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lppassword: ::windows::core::PCWSTR, lpusername: ::windows::core::PCWSTR, dwflags: NET_USE_CONNECT_FLAGS) -> u32;
    }
    NPAddConnection3(hwndowner.into(), ::core::mem::transmute(lpnetresource), lppassword.into(), lpusername.into(), dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NPAddConnection4<'a, P0>(hwndowner: P0, lpnetresource: *const NETRESOURCEW, lpauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NPAddConnection4(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lpauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32) -> u32;
    }
    NPAddConnection4(hwndowner.into(), ::core::mem::transmute(lpnetresource), ::core::mem::transmute(lpauthbuffer), cbauthbuffer, dwflags, ::core::mem::transmute(lpuseoptions), cbuseoptions)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NPCancelConnection<'a, P0, P1>(lpname: P0, fforce: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NPCancelConnection(lpname: ::windows::core::PCWSTR, fforce: super::super::Foundation::BOOL) -> u32;
    }
    NPCancelConnection(lpname.into(), fforce.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NPCancelConnection2<'a, P0, P1>(lpname: P0, fforce: P1, dwflags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NPCancelConnection2(lpname: ::windows::core::PCWSTR, fforce: super::super::Foundation::BOOL, dwflags: u32) -> u32;
    }
    NPCancelConnection2(lpname.into(), fforce.into(), dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NPCloseEnum<'a, P0>(henum: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NPCloseEnum(henum: super::super::Foundation::HANDLE) -> u32;
    }
    NPCloseEnum(henum.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NPDIRECTORY_NOTIFY_OPERATION(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNDN_MKDIR: NPDIRECTORY_NOTIFY_OPERATION = NPDIRECTORY_NOTIFY_OPERATION(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNDN_RMDIR: NPDIRECTORY_NOTIFY_OPERATION = NPDIRECTORY_NOTIFY_OPERATION(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNDN_MVDIR: NPDIRECTORY_NOTIFY_OPERATION = NPDIRECTORY_NOTIFY_OPERATION(3u32);
impl ::core::marker::Copy for NPDIRECTORY_NOTIFY_OPERATION {}
impl ::core::clone::Clone for NPDIRECTORY_NOTIFY_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NPDIRECTORY_NOTIFY_OPERATION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NPDIRECTORY_NOTIFY_OPERATION {
    type Abi = Self;
}
impl ::core::fmt::Debug for NPDIRECTORY_NOTIFY_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NPDIRECTORY_NOTIFY_OPERATION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NPEnumResource<'a, P0>(henum: P0, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NPEnumResource(henum: super::super::Foundation::HANDLE, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    }
    NPEnumResource(henum.into(), ::core::mem::transmute(lpccount), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpbuffersize))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPFormatNetworkName<'a, P0>(lpremotename: P0, lpformattedname: ::windows::core::PWSTR, lpnlength: *mut u32, dwflags: NETWORK_NAME_FORMAT_FLAGS, dwavecharperline: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NPFormatNetworkName(lpremotename: ::windows::core::PCWSTR, lpformattedname: ::windows::core::PWSTR, lpnlength: *mut u32, dwflags: NETWORK_NAME_FORMAT_FLAGS, dwavecharperline: u32) -> u32;
    }
    NPFormatNetworkName(lpremotename.into(), ::core::mem::transmute(lpformattedname), ::core::mem::transmute(lpnlength), dwflags, dwavecharperline)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPGetCaps(ndex: u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NPGetCaps(ndex: u32) -> u32;
    }
    NPGetCaps(ndex)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPGetConnection<'a, P0>(lplocalname: P0, lpremotename: ::windows::core::PWSTR, lpnbufferlen: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NPGetConnection(lplocalname: ::windows::core::PCWSTR, lpremotename: ::windows::core::PWSTR, lpnbufferlen: *mut u32) -> u32;
    }
    NPGetConnection(lplocalname.into(), ::core::mem::transmute(lpremotename), ::core::mem::transmute(lpnbufferlen))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPGetConnection3<'a, P0>(lplocalname: P0, dwlevel: u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NPGetConnection3(lplocalname: ::windows::core::PCWSTR, dwlevel: u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    }
    NPGetConnection3(lplocalname.into(), dwlevel, ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpbuffersize))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPGetConnectionPerformance<'a, P0>(lpremotename: P0, lpnetconnectinfo: *mut NETCONNECTINFOSTRUCT) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NPGetConnectionPerformance(lpremotename: ::windows::core::PCWSTR, lpnetconnectinfo: *mut NETCONNECTINFOSTRUCT) -> u32;
    }
    NPGetConnectionPerformance(lpremotename.into(), ::core::mem::transmute(lpnetconnectinfo))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPGetPersistentUseOptionsForConnection<'a, P0>(lpremotepath: P0, lpreaduseoptions: *const u8, cbreaduseoptions: u32, lpwriteuseoptions: *mut u8, lpsizewriteuseoptions: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NPGetPersistentUseOptionsForConnection(lpremotepath: ::windows::core::PCWSTR, lpreaduseoptions: *const u8, cbreaduseoptions: u32, lpwriteuseoptions: *mut u8, lpsizewriteuseoptions: *mut u32) -> u32;
    }
    NPGetPersistentUseOptionsForConnection(lpremotepath.into(), ::core::mem::transmute(lpreaduseoptions), cbreaduseoptions, ::core::mem::transmute(lpwriteuseoptions), ::core::mem::transmute(lpsizewriteuseoptions))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPGetResourceInformation(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32, lplpsystem: *mut ::windows::core::PWSTR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NPGetResourceInformation(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32, lplpsystem: *mut ::windows::core::PWSTR) -> u32;
    }
    NPGetResourceInformation(::core::mem::transmute(lpnetresource), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpbuffersize), ::core::mem::transmute(lplpsystem))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPGetResourceParent(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NPGetResourceParent(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    }
    NPGetResourceParent(::core::mem::transmute(lpnetresource), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpbuffersize))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPGetUniversalName<'a, P0>(lplocalpath: P0, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NPGetUniversalName(lplocalpath: ::windows::core::PCWSTR, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    }
    NPGetUniversalName(lplocalpath.into(), dwinfolevel, ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpbuffersize))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn NPGetUser<'a, P0>(lpname: P0, lpusername: ::windows::core::PWSTR, lpnbufferlen: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NPGetUser(lpname: ::windows::core::PCWSTR, lpusername: ::windows::core::PWSTR, lpnbufferlen: *mut u32) -> u32;
    }
    NPGetUser(lpname.into(), ::core::mem::transmute(lpusername), ::core::mem::transmute(lpnbufferlen))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NPOpenEnum(dwscope: u32, dwtype: u32, dwusage: u32, lpnetresource: *const NETRESOURCEW, lphenum: *mut super::super::Foundation::HANDLE) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn NPOpenEnum(dwscope: u32, dwtype: u32, dwusage: u32, lpnetresource: *const NETRESOURCEW, lphenum: *mut super::super::Foundation::HANDLE) -> u32;
    }
    NPOpenEnum(dwscope, dwtype, dwusage, ::core::mem::transmute(lpnetresource), ::core::mem::transmute(lphenum))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NP_PROPERTY_DIALOG_SELECTION(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNPS_FILE: NP_PROPERTY_DIALOG_SELECTION = NP_PROPERTY_DIALOG_SELECTION(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNPS_DIR: NP_PROPERTY_DIALOG_SELECTION = NP_PROPERTY_DIALOG_SELECTION(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNPS_MULT: NP_PROPERTY_DIALOG_SELECTION = NP_PROPERTY_DIALOG_SELECTION(2u32);
impl ::core::marker::Copy for NP_PROPERTY_DIALOG_SELECTION {}
impl ::core::clone::Clone for NP_PROPERTY_DIALOG_SELECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for NP_PROPERTY_DIALOG_SELECTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for NP_PROPERTY_DIALOG_SELECTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for NP_PROPERTY_DIALOG_SELECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NP_PROPERTY_DIALOG_SELECTION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NetEnumHandle(pub isize);
impl NetEnumHandle {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for NetEnumHandle {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NetEnumHandle {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NetEnumHandle {}
impl ::core::fmt::Debug for NetEnumHandle {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NetEnumHandle").field(&self.0).finish()
    }
}
impl ::core::convert::From<::core::option::Option<NetEnumHandle>> for NetEnumHandle {
    fn from(optional: ::core::option::Option<NetEnumHandle>) -> NetEnumHandle {
        optional.unwrap_or_default()
    }
}
unsafe impl ::windows::core::Abi for NetEnumHandle {
    type Abi = Self;
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_AddConnectNotify = ::core::option::Option<unsafe extern "system" fn(lpnotifyinfo: *mut NOTIFYINFO, lpaddinfo: *const NOTIFYADD) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_CancelConnectNotify = ::core::option::Option<unsafe extern "system" fn(lpnotifyinfo: *mut NOTIFYINFO, lpcancelinfo: *const NOTIFYCANCEL) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPAddConnection = ::core::option::Option<unsafe extern "system" fn(lpnetresource: *const NETRESOURCEW, lppassword: ::windows::core::PCWSTR, lpusername: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPAddConnection3 = ::core::option::Option<unsafe extern "system" fn(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lppassword: ::windows::core::PCWSTR, lpusername: ::windows::core::PCWSTR, dwflags: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPAddConnection4 = ::core::option::Option<unsafe extern "system" fn(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lpauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPCancelConnection = ::core::option::Option<unsafe extern "system" fn(lpname: ::windows::core::PCWSTR, fforce: super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPCancelConnection2 = ::core::option::Option<unsafe extern "system" fn(lpname: ::windows::core::PCWSTR, fforce: super::super::Foundation::BOOL, dwflags: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPCloseEnum = ::core::option::Option<unsafe extern "system" fn(henum: super::super::Foundation::HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPDeviceMode = ::core::option::Option<unsafe extern "system" fn(hparent: super::super::Foundation::HWND) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPDirectoryNotify = ::core::option::Option<unsafe extern "system" fn(hwnd: super::super::Foundation::HWND, lpdir: ::windows::core::PCWSTR, dwoper: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPEnumResource = ::core::option::Option<unsafe extern "system" fn(henum: super::super::Foundation::HANDLE, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPFMXEditPerm = ::core::option::Option<unsafe extern "system" fn(lpdrivename: ::windows::core::PCWSTR, hwndfmx: super::super::Foundation::HWND, ndialogtype: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPFMXGetPermCaps = ::core::option::Option<unsafe extern "system" fn(lpdrivename: ::windows::core::PCWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPFMXGetPermHelp = ::core::option::Option<unsafe extern "system" fn(lpdrivename: ::windows::core::PCWSTR, ndialogtype: u32, fdirectory: super::super::Foundation::BOOL, lpfilenamebuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32, lpnhelpcontext: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPFormatNetworkName = ::core::option::Option<unsafe extern "system" fn(lpremotename: ::windows::core::PCWSTR, lpformattedname: ::windows::core::PWSTR, lpnlength: *mut u32, dwflags: u32, dwavecharperline: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPGetCaps = ::core::option::Option<unsafe extern "system" fn(ndex: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPGetConnection = ::core::option::Option<unsafe extern "system" fn(lplocalname: ::windows::core::PCWSTR, lpremotename: ::windows::core::PWSTR, lpnbufferlen: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPGetConnection3 = ::core::option::Option<unsafe extern "system" fn(lplocalname: ::windows::core::PCWSTR, dwlevel: u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPGetConnectionPerformance = ::core::option::Option<unsafe extern "system" fn(lpremotename: ::windows::core::PCWSTR, lpnetconnectinfo: *mut NETCONNECTINFOSTRUCT) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPGetDirectoryType = ::core::option::Option<unsafe extern "system" fn(lpname: ::windows::core::PCWSTR, lptype: *const i32, bflushcache: super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPGetPersistentUseOptionsForConnection = ::core::option::Option<unsafe extern "system" fn(lpremotepath: ::windows::core::PCWSTR, lpreaduseoptions: *const u8, cbreaduseoptions: u32, lpwriteuseoptions: *mut u8, lpsizewriteuseoptions: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPGetPropertyText = ::core::option::Option<unsafe extern "system" fn(ibutton: u32, npropsel: u32, lpname: ::windows::core::PCWSTR, lpbuttonname: ::windows::core::PWSTR, nbuttonnamelen: u32, ntype: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPGetResourceInformation = ::core::option::Option<unsafe extern "system" fn(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32, lplpsystem: *mut ::windows::core::PWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPGetResourceParent = ::core::option::Option<unsafe extern "system" fn(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPGetUniversalName = ::core::option::Option<unsafe extern "system" fn(lplocalpath: ::windows::core::PCWSTR, dwinfolevel: u32, lpbuffer: *mut ::core::ffi::c_void, lpnbuffersize: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPGetUser = ::core::option::Option<unsafe extern "system" fn(lpname: ::windows::core::PCWSTR, lpusername: ::windows::core::PWSTR, lpnbufferlen: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPLogonNotify = ::core::option::Option<unsafe extern "system" fn(lplogonid: *const super::super::Foundation::LUID, lpauthentinfotype: ::windows::core::PCWSTR, lpauthentinfo: *const ::core::ffi::c_void, lppreviousauthentinfotype: ::windows::core::PCWSTR, lppreviousauthentinfo: *const ::core::ffi::c_void, lpstationname: ::windows::core::PCWSTR, stationhandle: *const ::core::ffi::c_void, lplogonscript: *mut ::windows::core::PWSTR) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPOpenEnum = ::core::option::Option<unsafe extern "system" fn(dwscope: u32, dwtype: u32, dwusage: u32, lpnetresource: *const NETRESOURCEW, lphenum: *mut super::super::Foundation::HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub type PF_NPPasswordChangeNotify = ::core::option::Option<unsafe extern "system" fn(lpauthentinfotype: ::windows::core::PCWSTR, lpauthentinfo: *const ::core::ffi::c_void, lppreviousauthentinfotype: ::windows::core::PCWSTR, lppreviousauthentinfo: *const ::core::ffi::c_void, lpstationname: ::windows::core::PCWSTR, stationhandle: *const ::core::ffi::c_void, dwchangeinfo: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPPropertyDialog = ::core::option::Option<unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, ibuttondlg: u32, npropsel: u32, lpfilename: ::windows::core::PCWSTR, ntype: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type PF_NPSearchDialog = ::core::option::Option<unsafe extern "system" fn(hwndparent: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, cbbuffer: u32, lpnflags: *mut u32) -> u32>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub struct REMOTE_NAME_INFOA {
    pub lpUniversalName: ::windows::core::PSTR,
    pub lpConnectionName: ::windows::core::PSTR,
    pub lpRemainingPath: ::windows::core::PSTR,
}
impl ::core::marker::Copy for REMOTE_NAME_INFOA {}
impl ::core::clone::Clone for REMOTE_NAME_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REMOTE_NAME_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REMOTE_NAME_INFOA").field("lpUniversalName", &self.lpUniversalName).field("lpConnectionName", &self.lpConnectionName).field("lpRemainingPath", &self.lpRemainingPath).finish()
    }
}
unsafe impl ::windows::core::Abi for REMOTE_NAME_INFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REMOTE_NAME_INFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REMOTE_NAME_INFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for REMOTE_NAME_INFOA {}
impl ::core::default::Default for REMOTE_NAME_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub struct REMOTE_NAME_INFOW {
    pub lpUniversalName: ::windows::core::PWSTR,
    pub lpConnectionName: ::windows::core::PWSTR,
    pub lpRemainingPath: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for REMOTE_NAME_INFOW {}
impl ::core::clone::Clone for REMOTE_NAME_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REMOTE_NAME_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REMOTE_NAME_INFOW").field("lpUniversalName", &self.lpUniversalName).field("lpConnectionName", &self.lpConnectionName).field("lpRemainingPath", &self.lpRemainingPath).finish()
    }
}
unsafe impl ::windows::core::Abi for REMOTE_NAME_INFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for REMOTE_NAME_INFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<REMOTE_NAME_INFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for REMOTE_NAME_INFOW {}
impl ::core::default::Default for REMOTE_NAME_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEDISPLAYTYPE_DIRECTORY: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEDISPLAYTYPE_NDSCONTAINER: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEDISPLAYTYPE_NETWORK: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEDISPLAYTYPE_ROOT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEDISPLAYTYPE_SHAREADMIN: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCETYPE_RESERVED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCETYPE_UNKNOWN: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEUSAGE_NOLOCALDEVICE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEUSAGE_RESERVED: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEUSAGE_SIBLING: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCE_RECENT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct UNC_INFO_LEVEL(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const UNIVERSAL_NAME_INFO_LEVEL: UNC_INFO_LEVEL = UNC_INFO_LEVEL(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const REMOTE_NAME_INFO_LEVEL: UNC_INFO_LEVEL = UNC_INFO_LEVEL(2u32);
impl ::core::marker::Copy for UNC_INFO_LEVEL {}
impl ::core::clone::Clone for UNC_INFO_LEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for UNC_INFO_LEVEL {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for UNC_INFO_LEVEL {
    type Abi = Self;
}
impl ::core::fmt::Debug for UNC_INFO_LEVEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UNC_INFO_LEVEL").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub struct UNIVERSAL_NAME_INFOA {
    pub lpUniversalName: ::windows::core::PSTR,
}
impl ::core::marker::Copy for UNIVERSAL_NAME_INFOA {}
impl ::core::clone::Clone for UNIVERSAL_NAME_INFOA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UNIVERSAL_NAME_INFOA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNIVERSAL_NAME_INFOA").field("lpUniversalName", &self.lpUniversalName).finish()
    }
}
unsafe impl ::windows::core::Abi for UNIVERSAL_NAME_INFOA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UNIVERSAL_NAME_INFOA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UNIVERSAL_NAME_INFOA>()) == 0 }
    }
}
impl ::core::cmp::Eq for UNIVERSAL_NAME_INFOA {}
impl ::core::default::Default for UNIVERSAL_NAME_INFOA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub struct UNIVERSAL_NAME_INFOW {
    pub lpUniversalName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for UNIVERSAL_NAME_INFOW {}
impl ::core::clone::Clone for UNIVERSAL_NAME_INFOW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UNIVERSAL_NAME_INFOW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UNIVERSAL_NAME_INFOW").field("lpUniversalName", &self.lpUniversalName).finish()
    }
}
unsafe impl ::windows::core::Abi for UNIVERSAL_NAME_INFOW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UNIVERSAL_NAME_INFOW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UNIVERSAL_NAME_INFOW>()) == 0 }
    }
}
impl ::core::cmp::Eq for UNIVERSAL_NAME_INFOW {}
impl ::core::default::Default for UNIVERSAL_NAME_INFOW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNCON_DYNAMIC: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNCON_FORNETCARD: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNCON_NOTROUTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNCON_SLOWLINK: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNDT_NETWORK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNDT_NORMAL: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WNET_OPEN_ENUM_USAGE(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEUSAGE_NONE: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEUSAGE_CONNECTABLE: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEUSAGE_CONTAINER: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(2u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEUSAGE_ATTACHED: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(16u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const RESOURCEUSAGE_ALL: WNET_OPEN_ENUM_USAGE = WNET_OPEN_ENUM_USAGE(19u32);
impl ::core::marker::Copy for WNET_OPEN_ENUM_USAGE {}
impl ::core::clone::Clone for WNET_OPEN_ENUM_USAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WNET_OPEN_ENUM_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WNET_OPEN_ENUM_USAGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WNET_OPEN_ENUM_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WNET_OPEN_ENUM_USAGE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for WNET_OPEN_ENUM_USAGE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for WNET_OPEN_ENUM_USAGE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for WNET_OPEN_ENUM_USAGE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for WNET_OPEN_ENUM_USAGE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for WNET_OPEN_ENUM_USAGE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNFMT_CONNECTION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNFMT_INENUM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNGETCON_CONNECTED: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNGETCON_DISCONNECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_ADMIN: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_ADM_DIRECTORYNOTIFY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_ADM_GETDIRECTORYTYPE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_CONNECTION: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_CONNECTION_FLAGS: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_CON_ADDCONNECTION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_CON_ADDCONNECTION3: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_CON_ADDCONNECTION4: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_CON_CANCELCONNECTION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_CON_CANCELCONNECTION2: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_CON_DEFER: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_CON_GETCONNECTIONS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_CON_GETPERFORMANCE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_DIALOG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_DLG_DEVICEMODE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_DLG_FORMATNETWORKNAME: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_DLG_GETRESOURCEINFORMATION: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_DLG_GETRESOURCEPARENT: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_DLG_PERMISSIONEDITOR: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_DLG_PROPERTYDIALOG: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_DLG_SEARCHDIALOG: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_DRIVER_VERSION: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_ENUMERATION: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_ENUM_CONTEXT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_ENUM_GLOBAL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_ENUM_LOCAL: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_ENUM_SHAREABLE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_NET_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_NET_TYPE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_SPEC_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_SPEC_VERSION51: u32 = 327681u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_START: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_USER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_USR_GETUSER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNNC_WAIT_FOR_START: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNPERMC_AUDIT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNPERMC_OWNER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNPERMC_PERM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WNPERM_DLG(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNPERM_DLG_PERM: WNPERM_DLG = WNPERM_DLG(0u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNPERM_DLG_AUDIT: WNPERM_DLG = WNPERM_DLG(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNPERM_DLG_OWNER: WNPERM_DLG = WNPERM_DLG(2u32);
impl ::core::marker::Copy for WNPERM_DLG {}
impl ::core::clone::Clone for WNPERM_DLG {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WNPERM_DLG {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WNPERM_DLG {
    type Abi = Self;
}
impl ::core::fmt::Debug for WNPERM_DLG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WNPERM_DLG").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNSRCH_REFRESH_FIRST_LEVEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNTYPE_COMM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNTYPE_DRIVE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNTYPE_FILE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WNTYPE_PRINTER: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WN_CREDENTIAL_CLASS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WN_NETWORK_CLASS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WN_NT_PASSWORD_CHANGED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WN_PRIMARY_AUTHENT_CLASS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WN_SERVICE_CLASS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
pub const WN_VALID_LOGON_ACCOUNT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetAddConnection2A<'a, P0, P1>(lpnetresource: *const NETRESOURCEA, lppassword: P0, lpusername: P1, dwflags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetAddConnection2A(lpnetresource: *const NETRESOURCEA, lppassword: ::windows::core::PCSTR, lpusername: ::windows::core::PCSTR, dwflags: u32) -> u32;
    }
    WNetAddConnection2A(::core::mem::transmute(lpnetresource), lppassword.into(), lpusername.into(), dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetAddConnection2W<'a, P0, P1>(lpnetresource: *const NETRESOURCEW, lppassword: P0, lpusername: P1, dwflags: u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetAddConnection2W(lpnetresource: *const NETRESOURCEW, lppassword: ::windows::core::PCWSTR, lpusername: ::windows::core::PCWSTR, dwflags: u32) -> u32;
    }
    WNetAddConnection2W(::core::mem::transmute(lpnetresource), lppassword.into(), lpusername.into(), dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetAddConnection3A<'a, P0, P1, P2>(hwndowner: P0, lpnetresource: *const NETRESOURCEA, lppassword: P1, lpusername: P2, dwflags: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetAddConnection3A(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEA, lppassword: ::windows::core::PCSTR, lpusername: ::windows::core::PCSTR, dwflags: u32) -> u32;
    }
    WNetAddConnection3A(hwndowner.into(), ::core::mem::transmute(lpnetresource), lppassword.into(), lpusername.into(), dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetAddConnection3W<'a, P0, P1, P2>(hwndowner: P0, lpnetresource: *const NETRESOURCEW, lppassword: P1, lpusername: P2, dwflags: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetAddConnection3W(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lppassword: ::windows::core::PCWSTR, lpusername: ::windows::core::PCWSTR, dwflags: u32) -> u32;
    }
    WNetAddConnection3W(hwndowner.into(), ::core::mem::transmute(lpnetresource), lppassword.into(), lpusername.into(), dwflags)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetAddConnection4A<'a, P0>(hwndowner: P0, lpnetresource: *const NETRESOURCEA, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetAddConnection4A(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEA, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32) -> u32;
    }
    WNetAddConnection4A(hwndowner.into(), ::core::mem::transmute(lpnetresource), ::core::mem::transmute(pauthbuffer), cbauthbuffer, dwflags, ::core::mem::transmute(lpuseoptions), cbuseoptions)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetAddConnection4W<'a, P0>(hwndowner: P0, lpnetresource: *const NETRESOURCEW, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetAddConnection4W(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32) -> u32;
    }
    WNetAddConnection4W(hwndowner.into(), ::core::mem::transmute(lpnetresource), ::core::mem::transmute(pauthbuffer), cbauthbuffer, dwflags, ::core::mem::transmute(lpuseoptions), cbuseoptions)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetAddConnectionA<'a, P0, P1, P2>(lpremotename: P0, lppassword: P1, lplocalname: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetAddConnectionA(lpremotename: ::windows::core::PCSTR, lppassword: ::windows::core::PCSTR, lplocalname: ::windows::core::PCSTR) -> u32;
    }
    WNetAddConnectionA(lpremotename.into(), lppassword.into(), lplocalname.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetAddConnectionW<'a, P0, P1, P2>(lpremotename: P0, lppassword: P1, lplocalname: P2) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetAddConnectionW(lpremotename: ::windows::core::PCWSTR, lppassword: ::windows::core::PCWSTR, lplocalname: ::windows::core::PCWSTR) -> u32;
    }
    WNetAddConnectionW(lpremotename.into(), lppassword.into(), lplocalname.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetCancelConnection2A<'a, P0, P1>(lpname: P0, dwflags: u32, fforce: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetCancelConnection2A(lpname: ::windows::core::PCSTR, dwflags: u32, fforce: super::super::Foundation::BOOL) -> u32;
    }
    WNetCancelConnection2A(lpname.into(), dwflags, fforce.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetCancelConnection2W<'a, P0, P1>(lpname: P0, dwflags: u32, fforce: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetCancelConnection2W(lpname: ::windows::core::PCWSTR, dwflags: u32, fforce: super::super::Foundation::BOOL) -> u32;
    }
    WNetCancelConnection2W(lpname.into(), dwflags, fforce.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetCancelConnectionA<'a, P0, P1>(lpname: P0, fforce: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetCancelConnectionA(lpname: ::windows::core::PCSTR, fforce: super::super::Foundation::BOOL) -> u32;
    }
    WNetCancelConnectionA(lpname.into(), fforce.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetCancelConnectionW<'a, P0, P1>(lpname: P0, fforce: P1) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<super::super::Foundation::BOOL>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetCancelConnectionW(lpname: ::windows::core::PCWSTR, fforce: super::super::Foundation::BOOL) -> u32;
    }
    WNetCancelConnectionW(lpname.into(), fforce.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetCloseEnum<'a, P0>(henum: P0) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetCloseEnum(henum: super::super::Foundation::HANDLE) -> u32;
    }
    WNetCloseEnum(henum.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetConnectionDialog<'a, P0>(hwnd: P0, dwtype: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetConnectionDialog(hwnd: super::super::Foundation::HWND, dwtype: u32) -> u32;
    }
    WNetConnectionDialog(hwnd.into(), dwtype)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetConnectionDialog1A(lpconndlgstruct: *mut CONNECTDLGSTRUCTA) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetConnectionDialog1A(lpconndlgstruct: *mut CONNECTDLGSTRUCTA) -> u32;
    }
    WNetConnectionDialog1A(::core::mem::transmute(lpconndlgstruct))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetConnectionDialog1W(lpconndlgstruct: *mut CONNECTDLGSTRUCTW) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetConnectionDialog1W(lpconndlgstruct: *mut CONNECTDLGSTRUCTW) -> u32;
    }
    WNetConnectionDialog1W(::core::mem::transmute(lpconndlgstruct))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetDisconnectDialog<'a, P0>(hwnd: P0, dwtype: u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetDisconnectDialog(hwnd: super::super::Foundation::HWND, dwtype: u32) -> u32;
    }
    WNetDisconnectDialog(hwnd.into(), dwtype)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetDisconnectDialog1A(lpconndlgstruct: *const DISCDLGSTRUCTA) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetDisconnectDialog1A(lpconndlgstruct: *const DISCDLGSTRUCTA) -> u32;
    }
    WNetDisconnectDialog1A(::core::mem::transmute(lpconndlgstruct))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetDisconnectDialog1W(lpconndlgstruct: *const DISCDLGSTRUCTW) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetDisconnectDialog1W(lpconndlgstruct: *const DISCDLGSTRUCTW) -> u32;
    }
    WNetDisconnectDialog1W(::core::mem::transmute(lpconndlgstruct))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetEnumResourceA<'a, P0>(henum: P0, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetEnumResourceA(henum: super::super::Foundation::HANDLE, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    }
    WNetEnumResourceA(henum.into(), ::core::mem::transmute(lpccount), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpbuffersize))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetEnumResourceW<'a, P0>(henum: P0, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetEnumResourceW(henum: super::super::Foundation::HANDLE, lpccount: *mut u32, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    }
    WNetEnumResourceW(henum.into(), ::core::mem::transmute(lpccount), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpbuffersize))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetConnectionA<'a, P0>(lplocalname: P0, lpremotename: ::windows::core::PSTR, lpnlength: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetGetConnectionA(lplocalname: ::windows::core::PCSTR, lpremotename: ::windows::core::PSTR, lpnlength: *mut u32) -> u32;
    }
    WNetGetConnectionA(lplocalname.into(), ::core::mem::transmute(lpremotename), ::core::mem::transmute(lpnlength))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetConnectionW<'a, P0>(lplocalname: P0, lpremotename: ::windows::core::PWSTR, lpnlength: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetGetConnectionW(lplocalname: ::windows::core::PCWSTR, lpremotename: ::windows::core::PWSTR, lpnlength: *mut u32) -> u32;
    }
    WNetGetConnectionW(lplocalname.into(), ::core::mem::transmute(lpremotename), ::core::mem::transmute(lpnlength))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetLastErrorA(lperror: *mut u32, lperrorbuf: &mut [u8], lpnamebuf: &mut [u8]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetGetLastErrorA(lperror: *mut u32, lperrorbuf: ::windows::core::PSTR, nerrorbufsize: u32, lpnamebuf: ::windows::core::PSTR, nnamebufsize: u32) -> u32;
    }
    WNetGetLastErrorA(::core::mem::transmute(lperror), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lperrorbuf)), lperrorbuf.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpnamebuf)), lpnamebuf.len() as _)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetLastErrorW(lperror: *mut u32, lperrorbuf: &mut [u16], lpnamebuf: &mut [u16]) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetGetLastErrorW(lperror: *mut u32, lperrorbuf: ::windows::core::PWSTR, nerrorbufsize: u32, lpnamebuf: ::windows::core::PWSTR, nnamebufsize: u32) -> u32;
    }
    WNetGetLastErrorW(::core::mem::transmute(lperror), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lperrorbuf)), lperrorbuf.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(lpnamebuf)), lpnamebuf.len() as _)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetGetNetworkInformationA<'a, P0>(lpprovider: P0, lpnetinfostruct: *mut NETINFOSTRUCT) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetGetNetworkInformationA(lpprovider: ::windows::core::PCSTR, lpnetinfostruct: *mut NETINFOSTRUCT) -> u32;
    }
    WNetGetNetworkInformationA(lpprovider.into(), ::core::mem::transmute(lpnetinfostruct))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetGetNetworkInformationW<'a, P0>(lpprovider: P0, lpnetinfostruct: *mut NETINFOSTRUCT) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetGetNetworkInformationW(lpprovider: ::windows::core::PCWSTR, lpnetinfostruct: *mut NETINFOSTRUCT) -> u32;
    }
    WNetGetNetworkInformationW(lpprovider.into(), ::core::mem::transmute(lpnetinfostruct))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetProviderNameA(dwnettype: u32, lpprovidername: ::windows::core::PSTR, lpbuffersize: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetGetProviderNameA(dwnettype: u32, lpprovidername: ::windows::core::PSTR, lpbuffersize: *mut u32) -> u32;
    }
    WNetGetProviderNameA(dwnettype, ::core::mem::transmute(lpprovidername), ::core::mem::transmute(lpbuffersize))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetProviderNameW(dwnettype: u32, lpprovidername: ::windows::core::PWSTR, lpbuffersize: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetGetProviderNameW(dwnettype: u32, lpprovidername: ::windows::core::PWSTR, lpbuffersize: *mut u32) -> u32;
    }
    WNetGetProviderNameW(dwnettype, ::core::mem::transmute(lpprovidername), ::core::mem::transmute(lpbuffersize))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetResourceInformationA(lpnetresource: *const NETRESOURCEA, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32, lplpsystem: *mut ::windows::core::PSTR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetGetResourceInformationA(lpnetresource: *const NETRESOURCEA, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32, lplpsystem: *mut ::windows::core::PSTR) -> u32;
    }
    WNetGetResourceInformationA(::core::mem::transmute(lpnetresource), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpcbbuffer), ::core::mem::transmute(lplpsystem))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetResourceInformationW(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32, lplpsystem: *mut ::windows::core::PWSTR) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetGetResourceInformationW(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32, lplpsystem: *mut ::windows::core::PWSTR) -> u32;
    }
    WNetGetResourceInformationW(::core::mem::transmute(lpnetresource), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpcbbuffer), ::core::mem::transmute(lplpsystem))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetResourceParentA(lpnetresource: *const NETRESOURCEA, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetGetResourceParentA(lpnetresource: *const NETRESOURCEA, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32) -> u32;
    }
    WNetGetResourceParentA(::core::mem::transmute(lpnetresource), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpcbbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetResourceParentW(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetGetResourceParentW(lpnetresource: *const NETRESOURCEW, lpbuffer: *mut ::core::ffi::c_void, lpcbbuffer: *mut u32) -> u32;
    }
    WNetGetResourceParentW(::core::mem::transmute(lpnetresource), ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpcbbuffer))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetUniversalNameA<'a, P0>(lplocalpath: P0, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetGetUniversalNameA(lplocalpath: ::windows::core::PCSTR, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    }
    WNetGetUniversalNameA(lplocalpath.into(), dwinfolevel, ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpbuffersize))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetUniversalNameW<'a, P0>(lplocalpath: P0, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetGetUniversalNameW(lplocalpath: ::windows::core::PCWSTR, dwinfolevel: UNC_INFO_LEVEL, lpbuffer: *mut ::core::ffi::c_void, lpbuffersize: *mut u32) -> u32;
    }
    WNetGetUniversalNameW(lplocalpath.into(), dwinfolevel, ::core::mem::transmute(lpbuffer), ::core::mem::transmute(lpbuffersize))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetUserA<'a, P0>(lpname: P0, lpusername: ::windows::core::PSTR, lpnlength: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetGetUserA(lpname: ::windows::core::PCSTR, lpusername: ::windows::core::PSTR, lpnlength: *mut u32) -> u32;
    }
    WNetGetUserA(lpname.into(), ::core::mem::transmute(lpusername), ::core::mem::transmute(lpnlength))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetGetUserW<'a, P0>(lpname: P0, lpusername: ::windows::core::PWSTR, lpnlength: *mut u32) -> u32
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetGetUserW(lpname: ::windows::core::PCWSTR, lpusername: ::windows::core::PWSTR, lpnlength: *mut u32) -> u32;
    }
    WNetGetUserW(lpname.into(), ::core::mem::transmute(lpusername), ::core::mem::transmute(lpnlength))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetOpenEnumA(dwscope: NET_RESOURCE_SCOPE, dwtype: NET_RESOURCE_TYPE, dwusage: WNET_OPEN_ENUM_USAGE, lpnetresource: *const NETRESOURCEA, lphenum: *mut NetEnumHandle) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetOpenEnumA(dwscope: NET_RESOURCE_SCOPE, dwtype: NET_RESOURCE_TYPE, dwusage: WNET_OPEN_ENUM_USAGE, lpnetresource: *const NETRESOURCEA, lphenum: *mut NetEnumHandle) -> u32;
    }
    WNetOpenEnumA(dwscope, dwtype, dwusage, ::core::mem::transmute(lpnetresource), ::core::mem::transmute(lphenum))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetOpenEnumW(dwscope: NET_RESOURCE_SCOPE, dwtype: NET_RESOURCE_TYPE, dwusage: WNET_OPEN_ENUM_USAGE, lpnetresource: *const NETRESOURCEW, lphenum: *mut NetEnumHandle) -> u32 {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetOpenEnumW(dwscope: NET_RESOURCE_SCOPE, dwtype: NET_RESOURCE_TYPE, dwusage: WNET_OPEN_ENUM_USAGE, lpnetresource: *const NETRESOURCEW, lphenum: *mut NetEnumHandle) -> u32;
    }
    WNetOpenEnumW(dwscope, dwtype, dwusage, ::core::mem::transmute(lpnetresource), ::core::mem::transmute(lphenum))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetSetLastErrorA<'a, P0, P1>(err: u32, lperror: P0, lpproviders: P1)
where
    P0: ::std::convert::Into<::windows::core::PCSTR>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetSetLastErrorA(err: u32, lperror: ::windows::core::PCSTR, lpproviders: ::windows::core::PCSTR);
    }
    WNetSetLastErrorA(err, lperror.into(), lpproviders.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`*"]
#[inline]
pub unsafe fn WNetSetLastErrorW<'a, P0, P1>(err: u32, lperror: P0, lpproviders: P1)
where
    P0: ::std::convert::Into<::windows::core::PCWSTR>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetSetLastErrorW(err: u32, lperror: ::windows::core::PCWSTR, lpproviders: ::windows::core::PCWSTR);
    }
    WNetSetLastErrorW(err, lperror.into(), lpproviders.into())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetUseConnection4A<'a, P0>(hwndowner: P0, lpnetresource: *const NETRESOURCEA, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32, lpaccessname: ::windows::core::PSTR, lpbuffersize: *mut u32, lpresult: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetUseConnection4A(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEA, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32, lpaccessname: ::windows::core::PSTR, lpbuffersize: *mut u32, lpresult: *mut u32) -> u32;
    }
    WNetUseConnection4A(hwndowner.into(), ::core::mem::transmute(lpnetresource), ::core::mem::transmute(pauthbuffer), cbauthbuffer, dwflags, ::core::mem::transmute(lpuseoptions), cbuseoptions, ::core::mem::transmute(lpaccessname), ::core::mem::transmute(lpbuffersize), ::core::mem::transmute(lpresult))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetUseConnection4W<'a, P0>(hwndowner: P0, lpnetresource: *const NETRESOURCEW, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32, lpaccessname: ::windows::core::PWSTR, lpbuffersize: *mut u32, lpresult: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetUseConnection4W(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, pauthbuffer: *const ::core::ffi::c_void, cbauthbuffer: u32, dwflags: u32, lpuseoptions: *const u8, cbuseoptions: u32, lpaccessname: ::windows::core::PWSTR, lpbuffersize: *mut u32, lpresult: *mut u32) -> u32;
    }
    WNetUseConnection4W(hwndowner.into(), ::core::mem::transmute(lpnetresource), ::core::mem::transmute(pauthbuffer), cbauthbuffer, dwflags, ::core::mem::transmute(lpuseoptions), cbuseoptions, ::core::mem::transmute(lpaccessname), ::core::mem::transmute(lpbuffersize), ::core::mem::transmute(lpresult))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetUseConnectionA<'a, P0, P1, P2>(hwndowner: P0, lpnetresource: *const NETRESOURCEA, lppassword: P1, lpuserid: P2, dwflags: NET_USE_CONNECT_FLAGS, lpaccessname: ::windows::core::PSTR, lpbuffersize: *mut u32, lpresult: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCSTR>,
    P2: ::std::convert::Into<::windows::core::PCSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetUseConnectionA(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEA, lppassword: ::windows::core::PCSTR, lpuserid: ::windows::core::PCSTR, dwflags: NET_USE_CONNECT_FLAGS, lpaccessname: ::windows::core::PSTR, lpbuffersize: *mut u32, lpresult: *mut u32) -> u32;
    }
    WNetUseConnectionA(hwndowner.into(), ::core::mem::transmute(lpnetresource), lppassword.into(), lpuserid.into(), dwflags, ::core::mem::transmute(lpaccessname), ::core::mem::transmute(lpbuffersize), ::core::mem::transmute(lpresult))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WNet\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WNetUseConnectionW<'a, P0, P1, P2>(hwndowner: P0, lpnetresource: *const NETRESOURCEW, lppassword: P1, lpuserid: P2, dwflags: NET_USE_CONNECT_FLAGS, lpaccessname: ::windows::core::PWSTR, lpbuffersize: *mut u32, lpresult: *mut u32) -> u32
where
    P0: ::std::convert::Into<super::super::Foundation::HWND>,
    P1: ::std::convert::Into<::windows::core::PCWSTR>,
    P2: ::std::convert::Into<::windows::core::PCWSTR>,
{
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn WNetUseConnectionW(hwndowner: super::super::Foundation::HWND, lpnetresource: *const NETRESOURCEW, lppassword: ::windows::core::PCWSTR, lpuserid: ::windows::core::PCWSTR, dwflags: NET_USE_CONNECT_FLAGS, lpaccessname: ::windows::core::PWSTR, lpbuffersize: *mut u32, lpresult: *mut u32) -> u32;
    }
    WNetUseConnectionW(hwndowner.into(), ::core::mem::transmute(lpnetresource), lppassword.into(), lpuserid.into(), dwflags, ::core::mem::transmute(lpaccessname), ::core::mem::transmute(lpbuffersize), ::core::mem::transmute(lpresult))
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
