#[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
#[repr(transparent)]
pub struct FileInformation(::windows::core::IUnknown);
impl FileInformation {
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenSequentialReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Streams::IInputStream>> {
        let this = &::windows::core::Interface::cast::<super::Streams::IInputStreamReference>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OpenSequentialReadAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::Streams::IInputStream>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenReadAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStreamWithContentType>> {
        let this = &::windows::core::Interface::cast::<super::Streams::IRandomAccessStreamReference>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OpenReadAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStreamWithContentType>>(result__)
        }
    }
    pub fn FileType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FileType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn ContentType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ContentType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenAsync(&self, accessmode: super::FileAccessMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStream>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OpenAsync)(::windows::core::Interface::as_raw(this), accessmode, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStream>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenTransactedWriteAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OpenTransactedWriteAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyOverloadDefaultNameAndOptions<'a, P0, E0>(&self, destinationfolder: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::IStorageFolder>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CopyOverloadDefaultNameAndOptions)(::windows::core::Interface::as_raw(this), destinationfolder.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyOverloadDefaultOptions<'a, P0, E0>(&self, destinationfolder: P0, desirednewname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::IStorageFolder>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CopyOverloadDefaultOptions)(::windows::core::Interface::as_raw(this), destinationfolder.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(desirednewname), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyOverload<'a, P0, E0>(&self, destinationfolder: P0, desirednewname: &::windows::core::HSTRING, option: super::NameCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::IStorageFolder>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CopyOverload)(::windows::core::Interface::as_raw(this), destinationfolder.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(desirednewname), option, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CopyAndReplaceAsync<'a, P0, E0>(&self, filetoreplace: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CopyAndReplaceAsync)(::windows::core::Interface::as_raw(this), filetoreplace.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveOverloadDefaultNameAndOptions<'a, P0, E0>(&self, destinationfolder: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::IStorageFolder>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MoveOverloadDefaultNameAndOptions)(::windows::core::Interface::as_raw(this), destinationfolder.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveOverloadDefaultOptions<'a, P0, E0>(&self, destinationfolder: P0, desirednewname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::IStorageFolder>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MoveOverloadDefaultOptions)(::windows::core::Interface::as_raw(this), destinationfolder.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(desirednewname), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveOverload<'a, P0, E0>(&self, destinationfolder: P0, desirednewname: &::windows::core::HSTRING, option: super::NameCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::IStorageFolder>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MoveOverload)(::windows::core::Interface::as_raw(this), destinationfolder.try_into().map_err(|e| e.into())?.abi(), ::core::mem::transmute_copy(desirednewname), option, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn MoveAndReplaceAsync<'a, P0, E0>(&self, filetoreplace: P0) -> ::windows::core::Result<super::super::Foundation::IAsyncAction>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::IStorageFile>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::IStorageFile>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MoveAndReplaceAsync)(::windows::core::Interface::as_raw(this), filetoreplace.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
    pub fn OpenWithOptionsAsync(&self, accessmode: super::FileAccessMode, options: super::StorageOpenOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStream>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OpenWithOptionsAsync)(::windows::core::Interface::as_raw(this), accessmode, options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::Streams::IRandomAccessStream>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn OpenTransactedWriteWithOptionsAsync(&self, options: super::StorageOpenOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFile2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).OpenTransactedWriteWithOptionsAsync)(::windows::core::Interface::as_raw(this), options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageStreamTransaction>>(result__)
        }
    }
    pub fn IsAvailable(&self) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IStorageFilePropertiesWithAvailability>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsAvailable)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsyncOverloadDefaultOptions(&self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RenameAsyncOverloadDefaultOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(desiredname), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsync(&self, desiredname: &::windows::core::HSTRING, option: super::NameCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RenameAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(desiredname), option, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsyncOverloadDefaultOptions)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self, option: super::StorageDeleteOption) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsync)(::windows::core::Interface::as_raw(this), option, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::BasicProperties>> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetBasicPropertiesAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::BasicProperties>>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Path)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<super::FileAttributes> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileAttributes>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DateCreated(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DateCreated)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn IsOfType(&self, r#type: super::StorageItemTypes) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsOfType)(::windows::core::Interface::as_raw(this), r#type, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetParentAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetParentAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    pub fn IsEqual<'a, P0, E0>(&self, item: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::IStorageItem>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsEqual)(::windows::core::Interface::as_raw(this), item.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn MusicProperties(&self) -> ::windows::core::Result<super::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MusicProperties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::MusicProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn VideoProperties(&self) -> ::windows::core::Result<super::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoProperties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::VideoProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn ImageProperties(&self) -> ::windows::core::Result<super::FileProperties::ImageProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImageProperties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::ImageProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn DocumentProperties(&self) -> ::windows::core::Result<super::FileProperties::DocumentProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DocumentProperties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::DocumentProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn BasicProperties(&self) -> ::windows::core::Result<super::FileProperties::BasicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BasicProperties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::BasicProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::FileProperties::StorageItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Thumbnail)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::StorageItemThumbnail>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ThumbnailUpdated<'a, P0>(&self, changedhandler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ThumbnailUpdated)(::windows::core::Interface::as_raw(this), changedhandler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveThumbnailUpdated(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveThumbnailUpdated)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PropertiesUpdated<'a, P0>(&self, changedhandler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PropertiesUpdated)(::windows::core::Interface::as_raw(this), changedhandler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePropertiesUpdated(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePropertiesUpdated)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultSizeDefaultOptions)(::windows::core::Interface::as_raw(this), mode, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultOptions)(::windows::core::Interface::as_raw(this), mode, requestedsize, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsync)(::windows::core::Interface::as_raw(this), mode, requestedsize, options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FolderRelativeId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FolderRelativeId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn Properties(&self) -> ::windows::core::Result<super::FileProperties::StorageItemContentProperties> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::StorageItemContentProperties>(result__)
        }
    }
    pub fn Provider(&self) -> ::windows::core::Result<super::StorageProvider> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemPropertiesWithProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Provider)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::StorageProvider>(result__)
        }
    }
}
impl ::core::clone::Clone for FileInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileInformation {}
impl ::core::fmt::Debug for FileInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.BulkAccess.FileInformation;{87a5cb8b-8972-4f40-8de0-d86fb179d8fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FileInformation {
    type Vtable = IStorageItemInformation_Vtbl;
    const IID: ::windows::core::GUID = <IStorageItemInformation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileInformation {
    const NAME: &'static str = "Windows.Storage.BulkAccess.FileInformation";
}
impl ::core::convert::From<FileInformation> for ::windows::core::IUnknown {
    fn from(value: FileInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileInformation> for ::windows::core::IUnknown {
    fn from(value: &FileInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FileInformation> for &::windows::core::IUnknown {
    fn from(value: &FileInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FileInformation> for ::windows::core::IInspectable {
    fn from(value: FileInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileInformation> for ::windows::core::IInspectable {
    fn from(value: &FileInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FileInformation> for &::windows::core::IInspectable {
    fn from(value: &FileInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<FileInformation> for super::Streams::IInputStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&FileInformation> for super::Streams::IInputStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::core::convert::TryFrom<&FileInformation> for ::windows::core::InParam<'a, super::Streams::IInputStreamReference> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<FileInformation> for super::Streams::IRandomAccessStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl ::core::convert::TryFrom<&FileInformation> for super::Streams::IRandomAccessStreamReference {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Streams")]
impl<'a> ::core::convert::TryFrom<&FileInformation> for ::windows::core::InParam<'a, super::Streams::IRandomAccessStreamReference> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageFile {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageFile {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileInformation> for ::windows::core::InParam<'a, super::IStorageFile> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageFile2 {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageFile2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileInformation> for ::windows::core::InParam<'a, super::IStorageFile2> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageFilePropertiesWithAvailability {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageFilePropertiesWithAvailability {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileInformation> for ::windows::core::InParam<'a, super::IStorageFilePropertiesWithAvailability> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageItem {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageItem {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileInformation> for ::windows::core::InParam<'a, super::IStorageItem> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageItem2 {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageItem2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileInformation> for ::windows::core::InParam<'a, super::IStorageItem2> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileInformation> for IStorageItemInformation {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for IStorageItemInformation {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileInformation> for ::windows::core::InParam<'a, IStorageItemInformation> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageItemProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageItemProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileInformation> for ::windows::core::InParam<'a, super::IStorageItemProperties> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FileInformation> for super::IStorageItemPropertiesWithProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: FileInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileInformation> for super::IStorageItemPropertiesWithProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FileInformation> for ::windows::core::InParam<'a, super::IStorageItemPropertiesWithProvider> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FileInformation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
#[repr(transparent)]
pub struct FileInformationFactory(::windows::core::IUnknown);
impl FileInformationFactory {
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetItemsAsync)(::windows::core::Interface::as_raw(this), startindex, maxitemstoretrieve, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetItemsAsyncDefaultStartAndCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<IStorageItemInformation>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetFilesAsync)(::windows::core::Interface::as_raw(this), startindex, maxitemstoretrieve, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetFilesAsyncDefaultStartAndCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FileInformation>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetFoldersAsync)(::windows::core::Interface::as_raw(this), startindex, maxitemstoretrieve, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsyncDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetFoldersAsyncDefaultStartAndCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<FolderInformation>>>(result__)
        }
    }
    pub fn GetVirtualizedItemsVector(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetVirtualizedItemsVector)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetVirtualizedFilesVector(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetVirtualizedFilesVector)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    pub fn GetVirtualizedFoldersVector(&self) -> ::windows::core::Result<::windows::core::IInspectable> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetVirtualizedFoldersVector)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::IInspectable>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub fn CreateWithMode<'a, P0, E0>(queryresult: P0, mode: super::FileProperties::ThumbnailMode) -> ::windows::core::Result<FileInformationFactory>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::Search::IStorageQueryResultBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithMode)(::windows::core::Interface::as_raw(this), queryresult.try_into().map_err(|e| e.into())?.abi(), mode, result__.as_mut_ptr()).from_abi::<FileInformationFactory>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub fn CreateWithModeAndSize<'a, P0, E0>(queryresult: P0, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32) -> ::windows::core::Result<FileInformationFactory>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::Search::IStorageQueryResultBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithModeAndSize)(::windows::core::Interface::as_raw(this), queryresult.try_into().map_err(|e| e.into())?.abi(), mode, requestedthumbnailsize, result__.as_mut_ptr()).from_abi::<FileInformationFactory>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub fn CreateWithModeAndSizeAndOptions<'a, P0, E0>(queryresult: P0, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions) -> ::windows::core::Result<FileInformationFactory>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::Search::IStorageQueryResultBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithModeAndSizeAndOptions)(::windows::core::Interface::as_raw(this), queryresult.try_into().map_err(|e| e.into())?.abi(), mode, requestedthumbnailsize, thumbnailoptions, result__.as_mut_ptr()).from_abi::<FileInformationFactory>(result__)
        })
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub fn CreateWithModeAndSizeAndOptionsAndFlags<'a, P0, E0>(queryresult: P0, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions, delayload: bool) -> ::windows::core::Result<FileInformationFactory>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::Search::IStorageQueryResultBase>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        Self::IFileInformationFactoryFactory(|this| unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateWithModeAndSizeAndOptionsAndFlags)(::windows::core::Interface::as_raw(this), queryresult.try_into().map_err(|e| e.into())?.abi(), mode, requestedthumbnailsize, thumbnailoptions, delayload, result__.as_mut_ptr()).from_abi::<FileInformationFactory>(result__)
        })
    }
    #[doc(hidden)]
    pub fn IFileInformationFactoryFactory<R, F: FnOnce(&IFileInformationFactoryFactory) -> ::windows::core::Result<R>>(callback: F) -> ::windows::core::Result<R> {
        static SHARED: ::windows::core::FactoryCache<FileInformationFactory, IFileInformationFactoryFactory> = ::windows::core::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl ::core::clone::Clone for FileInformationFactory {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FileInformationFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FileInformationFactory {}
impl ::core::fmt::Debug for FileInformationFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FileInformationFactory").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FileInformationFactory {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.BulkAccess.FileInformationFactory;{401d88be-960f-4d6d-a7d0-1a3861e76c83})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FileInformationFactory {
    type Vtable = IFileInformationFactory_Vtbl;
    const IID: ::windows::core::GUID = <IFileInformationFactory as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FileInformationFactory {
    const NAME: &'static str = "Windows.Storage.BulkAccess.FileInformationFactory";
}
impl ::core::convert::From<FileInformationFactory> for ::windows::core::IUnknown {
    fn from(value: FileInformationFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileInformationFactory> for ::windows::core::IUnknown {
    fn from(value: &FileInformationFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FileInformationFactory> for &::windows::core::IUnknown {
    fn from(value: &FileInformationFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FileInformationFactory> for ::windows::core::IInspectable {
    fn from(value: FileInformationFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FileInformationFactory> for ::windows::core::IInspectable {
    fn from(value: &FileInformationFactory) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FileInformationFactory> for &::windows::core::IInspectable {
    fn from(value: &FileInformationFactory) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
unsafe impl ::core::marker::Send for FileInformationFactory {}
unsafe impl ::core::marker::Sync for FileInformationFactory {}
#[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
#[repr(transparent)]
pub struct FolderInformation(::windows::core::IUnknown);
impl FolderInformation {
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFileAsyncOverloadDefaultOptions(&self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileAsyncOverloadDefaultOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(desiredname), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFileAsync(&self, desiredname: &::windows::core::HSTRING, options: super::CreationCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(desiredname), options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFolderAsyncOverloadDefaultOptions(&self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderAsyncOverloadDefaultOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(desiredname), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn CreateFolderAsync(&self, desiredname: &::windows::core::HSTRING, options: super::CreationCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(desiredname), options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFileAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFile>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetFileAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetFolderAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetFolderAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetItemAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetItemAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFilesAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetFilesAsyncOverloadDefaultOptionsStartAndCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetFoldersAsyncOverloadDefaultOptionsStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetFoldersAsyncOverloadDefaultOptionsStartAndCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`*"]
    #[cfg(feature = "Foundation_Collections")]
    pub fn GetItemsAsyncOverloadDefaultStartAndCount(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetItemsAsyncOverloadDefaultStartAndCount)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn TryGetItemAsync(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::IStorageItem>> {
        let this = &::windows::core::Interface::cast::<super::IStorageFolder2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).TryGetItemAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(name), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::IStorageItem>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_Search"))]
    pub fn GetIndexedStateAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::Search::IndexedState>> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetIndexedStateAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::Search::IndexedState>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFileQueryOverloadDefault(&self) -> ::windows::core::Result<super::Search::StorageFileQueryResult> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileQueryOverloadDefault)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Search::StorageFileQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFileQuery(&self, query: super::Search::CommonFileQuery) -> ::windows::core::Result<super::Search::StorageFileQueryResult> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileQuery)(::windows::core::Interface::as_raw(this), query, result__.as_mut_ptr()).from_abi::<super::Search::StorageFileQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFileQueryWithOptions<'a, P0>(&self, queryoptions: P0) -> ::windows::core::Result<super::Search::StorageFileQueryResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Search::QueryOptions>>,
    {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateFileQueryWithOptions)(::windows::core::Interface::as_raw(this), queryoptions.into().abi(), result__.as_mut_ptr()).from_abi::<super::Search::StorageFileQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFolderQueryOverloadDefault(&self) -> ::windows::core::Result<super::Search::StorageFolderQueryResult> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderQueryOverloadDefault)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Search::StorageFolderQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFolderQuery(&self, query: super::Search::CommonFolderQuery) -> ::windows::core::Result<super::Search::StorageFolderQueryResult> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderQuery)(::windows::core::Interface::as_raw(this), query, result__.as_mut_ptr()).from_abi::<super::Search::StorageFolderQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateFolderQueryWithOptions<'a, P0>(&self, queryoptions: P0) -> ::windows::core::Result<super::Search::StorageFolderQueryResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Search::QueryOptions>>,
    {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateFolderQueryWithOptions)(::windows::core::Interface::as_raw(this), queryoptions.into().abi(), result__.as_mut_ptr()).from_abi::<super::Search::StorageFolderQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateItemQuery(&self) -> ::windows::core::Result<super::Search::StorageItemQueryResult> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateItemQuery)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::Search::StorageItemQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn CreateItemQueryWithOptions<'a, P0>(&self, queryoptions: P0) -> ::windows::core::Result<super::Search::StorageItemQueryResult>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Search::QueryOptions>>,
    {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).CreateItemQueryWithOptions)(::windows::core::Interface::as_raw(this), queryoptions.into().abi(), result__.as_mut_ptr()).from_abi::<super::Search::StorageItemQueryResult>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetFilesAsync(&self, query: super::Search::CommonFileQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetFilesAsync)(::windows::core::Interface::as_raw(this), query, startindex, maxitemstoretrieve, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetFilesAsyncOverloadDefaultStartAndCount(&self, query: super::Search::CommonFileQuery) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetFilesAsyncOverloadDefaultStartAndCount)(::windows::core::Interface::as_raw(this), query, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFile>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetFoldersAsync(&self, query: super::Search::CommonFolderQuery, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetFoldersAsync)(::windows::core::Interface::as_raw(this), query, startindex, maxitemstoretrieve, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetFoldersAsyncOverloadDefaultStartAndCount(&self, query: super::Search::CommonFolderQuery) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetFoldersAsyncOverloadDefaultStartAndCount)(::windows::core::Interface::as_raw(this), query, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::StorageFolder>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation_Collections\"`, `\"Storage_Search\"`*"]
    #[cfg(all(feature = "Foundation_Collections", feature = "Storage_Search"))]
    pub fn GetItemsAsync(&self, startindex: u32, maxitemstoretrieve: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetItemsAsync)(::windows::core::Interface::as_raw(this), startindex, maxitemstoretrieve, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::super::Foundation::Collections::IVectorView<super::IStorageItem>>>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn AreQueryOptionsSupported<'a, P0>(&self, queryoptions: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::Search::QueryOptions>>,
    {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).AreQueryOptionsSupported)(::windows::core::Interface::as_raw(this), queryoptions.into().abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn IsCommonFolderQuerySupported(&self, query: super::Search::CommonFolderQuery) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsCommonFolderQuerySupported)(::windows::core::Interface::as_raw(this), query, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_Search\"`*"]
    #[cfg(feature = "Storage_Search")]
    pub fn IsCommonFileQuerySupported(&self, query: super::Search::CommonFileQuery) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::Search::IStorageFolderQueryOperations>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsCommonFileQuerySupported)(::windows::core::Interface::as_raw(this), query, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsyncOverloadDefaultOptions(&self, desiredname: &::windows::core::HSTRING) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RenameAsyncOverloadDefaultOptions)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(desiredname), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RenameAsync(&self, desiredname: &::windows::core::HSTRING, option: super::NameCollisionOption) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).RenameAsync)(::windows::core::Interface::as_raw(this), ::core::mem::transmute_copy(desiredname), option, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsyncOverloadDefaultOptions(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsyncOverloadDefaultOptions)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DeleteAsync(&self, option: super::StorageDeleteOption) -> ::windows::core::Result<super::super::Foundation::IAsyncAction> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DeleteAsync)(::windows::core::Interface::as_raw(this), option, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncAction>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties"))]
    pub fn GetBasicPropertiesAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::BasicProperties>> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetBasicPropertiesAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::BasicProperties>>(result__)
        }
    }
    pub fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Name)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Path(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Path)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn Attributes(&self) -> ::windows::core::Result<super::FileAttributes> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Attributes)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileAttributes>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn DateCreated(&self) -> ::windows::core::Result<super::super::Foundation::DateTime> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DateCreated)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::DateTime>(result__)
        }
    }
    pub fn IsOfType(&self, r#type: super::StorageItemTypes) -> ::windows::core::Result<bool> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsOfType)(::windows::core::Interface::as_raw(this), r#type, result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn GetParentAsync(&self) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::StorageFolder>> {
        let this = &::windows::core::Interface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetParentAsync)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::StorageFolder>>(result__)
        }
    }
    pub fn IsEqual<'a, P0, E0>(&self, item: P0) -> ::windows::core::Result<bool>
    where
        P0: ::std::convert::TryInto<::windows::core::InParam<'a, super::IStorageItem>, Error = E0>,
        E0: ::std::convert::Into<::windows::core::Error>,
    {
        let this = &::windows::core::Interface::cast::<super::IStorageItem2>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).IsEqual)(::windows::core::Interface::as_raw(this), item.try_into().map_err(|e| e.into())?.abi(), result__.as_mut_ptr()).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn MusicProperties(&self) -> ::windows::core::Result<super::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MusicProperties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::MusicProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn VideoProperties(&self) -> ::windows::core::Result<super::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoProperties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::VideoProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn ImageProperties(&self) -> ::windows::core::Result<super::FileProperties::ImageProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImageProperties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::ImageProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn DocumentProperties(&self) -> ::windows::core::Result<super::FileProperties::DocumentProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DocumentProperties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::DocumentProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn BasicProperties(&self) -> ::windows::core::Result<super::FileProperties::BasicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BasicProperties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::BasicProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::FileProperties::StorageItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Thumbnail)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::StorageItemThumbnail>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ThumbnailUpdated<'a, P0>(&self, changedhandler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ThumbnailUpdated)(::windows::core::Interface::as_raw(this), changedhandler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveThumbnailUpdated(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveThumbnailUpdated)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PropertiesUpdated<'a, P0>(&self, changedhandler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PropertiesUpdated)(::windows::core::Interface::as_raw(this), changedhandler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePropertiesUpdated(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePropertiesUpdated)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultSizeDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultSizeDefaultOptions)(::windows::core::Interface::as_raw(this), mode, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsyncOverloadDefaultOptions(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsyncOverloadDefaultOptions)(::windows::core::Interface::as_raw(this), mode, requestedsize, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`, `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Foundation", feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn GetThumbnailAsync(&self, mode: super::FileProperties::ThumbnailMode, requestedsize: u32, options: super::FileProperties::ThumbnailOptions) -> ::windows::core::Result<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).GetThumbnailAsync)(::windows::core::Interface::as_raw(this), mode, requestedsize, options, result__.as_mut_ptr()).from_abi::<super::super::Foundation::IAsyncOperation<super::FileProperties::StorageItemThumbnail>>(result__)
        }
    }
    pub fn DisplayName(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayName)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn DisplayType(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DisplayType)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    pub fn FolderRelativeId(&self) -> ::windows::core::Result<::windows::core::HSTRING> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).FolderRelativeId)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<::windows::core::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn Properties(&self) -> ::windows::core::Result<super::FileProperties::StorageItemContentProperties> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemProperties>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Properties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::StorageItemContentProperties>(result__)
        }
    }
    pub fn Provider(&self) -> ::windows::core::Result<super::StorageProvider> {
        let this = &::windows::core::Interface::cast::<super::IStorageItemPropertiesWithProvider>(self)?;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Provider)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::StorageProvider>(result__)
        }
    }
}
impl ::core::clone::Clone for FolderInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for FolderInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for FolderInformation {}
impl ::core::fmt::Debug for FolderInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FolderInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for FolderInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"rc(Windows.Storage.BulkAccess.FolderInformation;{87a5cb8b-8972-4f40-8de0-d86fb179d8fa})");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for FolderInformation {
    type Vtable = IStorageItemInformation_Vtbl;
    const IID: ::windows::core::GUID = <IStorageItemInformation as ::windows::core::Interface>::IID;
}
impl ::windows::core::RuntimeName for FolderInformation {
    const NAME: &'static str = "Windows.Storage.BulkAccess.FolderInformation";
}
impl ::core::convert::From<FolderInformation> for ::windows::core::IUnknown {
    fn from(value: FolderInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FolderInformation> for ::windows::core::IUnknown {
    fn from(value: &FolderInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FolderInformation> for &::windows::core::IUnknown {
    fn from(value: &FolderInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<FolderInformation> for ::windows::core::IInspectable {
    fn from(value: FolderInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&FolderInformation> for ::windows::core::IInspectable {
    fn from(value: &FolderInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<&FolderInformation> for &::windows::core::IInspectable {
    fn from(value: &FolderInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::TryFrom<FolderInformation> for super::IStorageFolder {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for super::IStorageFolder {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FolderInformation> for ::windows::core::InParam<'a, super::IStorageFolder> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FolderInformation> for super::IStorageFolder2 {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for super::IStorageFolder2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FolderInformation> for ::windows::core::InParam<'a, super::IStorageFolder2> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[cfg(feature = "Storage_Search")]
impl ::core::convert::TryFrom<FolderInformation> for super::Search::IStorageFolderQueryOperations {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
#[cfg(feature = "Storage_Search")]
impl ::core::convert::TryFrom<&FolderInformation> for super::Search::IStorageFolderQueryOperations {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
#[cfg(feature = "Storage_Search")]
impl<'a> ::core::convert::TryFrom<&FolderInformation> for ::windows::core::InParam<'a, super::Search::IStorageFolderQueryOperations> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FolderInformation> for super::IStorageItem {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for super::IStorageItem {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FolderInformation> for ::windows::core::InParam<'a, super::IStorageItem> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FolderInformation> for super::IStorageItem2 {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for super::IStorageItem2 {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FolderInformation> for ::windows::core::InParam<'a, super::IStorageItem2> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FolderInformation> for IStorageItemInformation {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for IStorageItemInformation {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FolderInformation> for ::windows::core::InParam<'a, IStorageItemInformation> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FolderInformation> for super::IStorageItemProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for super::IStorageItemProperties {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FolderInformation> for ::windows::core::InParam<'a, super::IStorageItemProperties> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
impl ::core::convert::TryFrom<FolderInformation> for super::IStorageItemPropertiesWithProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: FolderInformation) -> ::windows::core::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FolderInformation> for super::IStorageItemPropertiesWithProvider {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        ::windows::core::Interface::cast(value)
    }
}
impl<'a> ::core::convert::TryFrom<&FolderInformation> for ::windows::core::InParam<'a, super::IStorageItemPropertiesWithProvider> {
    type Error = ::windows::core::Error;
    fn try_from(value: &FolderInformation) -> ::windows::core::Result<Self> {
        let item = ::std::convert::TryInto::try_into(value)?;
        Ok(::windows::core::InParam::owned(item))
    }
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileInformationFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileInformationFactory {
    type Vtable = IFileInformationFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x401d88be_960f_4d6d_a7d0_1a3861e76c83);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileInformationFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetItemsAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetItemsAsyncDefaultStartAndCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFilesAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFilesAsyncDefaultStartAndCount: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsync: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, startindex: u32, maxitemstoretrieve: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsync: usize,
    #[cfg(feature = "Foundation_Collections")]
    pub GetFoldersAsyncDefaultStartAndCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation_Collections"))]
    GetFoldersAsyncDefaultStartAndCount: usize,
    pub GetVirtualizedItemsVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetVirtualizedFilesVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetVirtualizedFoldersVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IFileInformationFactoryFactory(::windows::core::IUnknown);
unsafe impl ::windows::core::Interface for IFileInformationFactoryFactory {
    type Vtable = IFileInformationFactoryFactory_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x84ea0e7d_e4a2_4f00_8afa_af5e0f826bd5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileInformationFactoryFactory_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub CreateWithMode: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryresult: *mut ::core::ffi::c_void, mode: super::FileProperties::ThumbnailMode, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))]
    CreateWithMode: usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub CreateWithModeAndSize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryresult: *mut ::core::ffi::c_void, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))]
    CreateWithModeAndSize: usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub CreateWithModeAndSizeAndOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryresult: *mut ::core::ffi::c_void, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))]
    CreateWithModeAndSizeAndOptions: usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Search"))]
    pub CreateWithModeAndSizeAndOptionsAndFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, queryresult: *mut ::core::ffi::c_void, mode: super::FileProperties::ThumbnailMode, requestedthumbnailsize: u32, thumbnailoptions: super::FileProperties::ThumbnailOptions, delayload: bool, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Search")))]
    CreateWithModeAndSizeAndOptionsAndFlags: usize,
}
#[doc = "*Required features: `\"Storage_BulkAccess\"`*"]
#[repr(transparent)]
pub struct IStorageItemInformation(::windows::core::IUnknown);
impl IStorageItemInformation {
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn MusicProperties(&self) -> ::windows::core::Result<super::FileProperties::MusicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).MusicProperties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::MusicProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn VideoProperties(&self) -> ::windows::core::Result<super::FileProperties::VideoProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).VideoProperties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::VideoProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn ImageProperties(&self) -> ::windows::core::Result<super::FileProperties::ImageProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ImageProperties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::ImageProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn DocumentProperties(&self) -> ::windows::core::Result<super::FileProperties::DocumentProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).DocumentProperties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::DocumentProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`*"]
    #[cfg(feature = "Storage_FileProperties")]
    pub fn BasicProperties(&self) -> ::windows::core::Result<super::FileProperties::BasicProperties> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).BasicProperties)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::BasicProperties>(result__)
        }
    }
    #[doc = "*Required features: `\"Storage_FileProperties\"`, `\"Storage_Streams\"`*"]
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub fn Thumbnail(&self) -> ::windows::core::Result<super::FileProperties::StorageItemThumbnail> {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).Thumbnail)(::windows::core::Interface::as_raw(this), result__.as_mut_ptr()).from_abi::<super::FileProperties::StorageItemThumbnail>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn ThumbnailUpdated<'a, P0>(&self, changedhandler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).ThumbnailUpdated)(::windows::core::Interface::as_raw(this), changedhandler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemoveThumbnailUpdated(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemoveThumbnailUpdated)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn PropertiesUpdated<'a, P0>(&self, changedhandler: P0) -> ::windows::core::Result<super::super::Foundation::EventRegistrationToken>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::Foundation::TypedEventHandler<IStorageItemInformation, ::windows::core::IInspectable>>>,
    {
        let this = self;
        unsafe {
            let mut result__ = ::core::mem::MaybeUninit::zeroed();
            (::windows::core::Interface::vtable(this).PropertiesUpdated)(::windows::core::Interface::as_raw(this), changedhandler.into().abi(), result__.as_mut_ptr()).from_abi::<super::super::Foundation::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `\"Foundation\"`*"]
    #[cfg(feature = "Foundation")]
    pub fn RemovePropertiesUpdated(&self, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::Result<()> {
        let this = self;
        unsafe { (::windows::core::Interface::vtable(this).RemovePropertiesUpdated)(::windows::core::Interface::as_raw(this), eventcookie).ok() }
    }
}
impl ::core::convert::From<IStorageItemInformation> for ::windows::core::IUnknown {
    fn from(value: IStorageItemInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IStorageItemInformation> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IStorageItemInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItemInformation> for ::windows::core::IUnknown {
    fn from(value: &IStorageItemInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IStorageItemInformation> for ::windows::core::IInspectable {
    fn from(value: IStorageItemInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IStorageItemInformation> for &'a ::windows::core::IInspectable {
    fn from(value: &'a IStorageItemInformation) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IStorageItemInformation> for ::windows::core::IInspectable {
    fn from(value: &IStorageItemInformation) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IStorageItemInformation {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IStorageItemInformation {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IStorageItemInformation {}
impl ::core::fmt::Debug for IStorageItemInformation {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IStorageItemInformation").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::RuntimeType for IStorageItemInformation {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"{87a5cb8b-8972-4f40-8de0-d86fb179d8fa}");
    type DefaultType = ::core::option::Option<Self>;
    fn from_default(from: &Self::DefaultType) -> ::windows::core::Result<Self> {
        from.as_ref().cloned().ok_or(::windows::core::Error::OK)
    }
}
unsafe impl ::windows::core::Interface for IStorageItemInformation {
    type Vtable = IStorageItemInformation_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x87a5cb8b_8972_4f40_8de0_d86fb179d8fa);
}
#[repr(C)]
#[doc(hidden)]
pub struct IStorageItemInformation_Vtbl {
    pub base__: ::windows::core::IInspectableVtbl,
    #[cfg(feature = "Storage_FileProperties")]
    pub MusicProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    MusicProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub VideoProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    VideoProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub ImageProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    ImageProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub DocumentProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    DocumentProperties: usize,
    #[cfg(feature = "Storage_FileProperties")]
    pub BasicProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Storage_FileProperties"))]
    BasicProperties: usize,
    #[cfg(all(feature = "Storage_FileProperties", feature = "Storage_Streams"))]
    pub Thumbnail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Storage_FileProperties", feature = "Storage_Streams")))]
    Thumbnail: usize,
    #[cfg(feature = "Foundation")]
    pub ThumbnailUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    ThumbnailUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemoveThumbnailUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemoveThumbnailUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub PropertiesUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changedhandler: *mut ::core::ffi::c_void, result__: *mut super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    PropertiesUpdated: usize,
    #[cfg(feature = "Foundation")]
    pub RemovePropertiesUpdated: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventcookie: super::super::Foundation::EventRegistrationToken) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Foundation"))]
    RemovePropertiesUpdated: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
