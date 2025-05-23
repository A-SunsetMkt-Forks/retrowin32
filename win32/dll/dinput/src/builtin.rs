#![doc = r" Generated code, do not edit.  See winapi/builtin.rs for an overview."]
#![allow(unused_imports)]
#![allow(unused_variables)]
use win32_system::dll::*;
mod wrappers {
    use crate as dinput;
    use crate::*;
    use ::memory::Extensions;
    use win32_system::{System, trace};
    use win32_winapi::{calling_convention::*, *};
    pub unsafe fn DirectInputCreateA(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let hinst = <u32>::from_stack(mem, stack_args + 0u32);
            let version = <u32>::from_stack(mem, stack_args + 4u32);
            let ppDI = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("dinput") {
                trace::Record::new(
                    dinput::DirectInputCreateA_pos,
                    "dinput",
                    "DirectInputCreateA",
                    &[
                        ("hinst", &hinst),
                        ("version", &version),
                        ("ppDI", &ppDI),
                        ("pUnkOuter", &pUnkOuter),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = dinput::DirectInputCreateA(sys, hinst, version, ppDI, pUnkOuter);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectInputDevice_Acquire(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("dinput") {
                trace::Record::new(
                    dinput::IDirectInputDevice::Acquire_pos,
                    "dinput",
                    "IDirectInputDevice::Acquire",
                    &[("this", &this)],
                )
                .enter()
            } else {
                None
            };
            let result = dinput::IDirectInputDevice::Acquire(sys, this);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectInputDevice_EnumObjects(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpCallback = <u32>::from_stack(mem, stack_args + 4u32);
            let pvRef = <u32>::from_stack(mem, stack_args + 8u32);
            let dwFlag = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("dinput") {
                trace::Record::new(
                    dinput::IDirectInputDevice::EnumObjects_pos,
                    "dinput",
                    "IDirectInputDevice::EnumObjects",
                    &[
                        ("this", &this),
                        ("lpCallback", &lpCallback),
                        ("pvRef", &pvRef),
                        ("dwFlag", &dwFlag),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result =
                dinput::IDirectInputDevice::EnumObjects(sys, this, lpCallback, pvRef, dwFlag);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectInputDevice_GetDeviceData(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let cbObjectData = <u32>::from_stack(mem, stack_args + 4u32);
            let rgdod = <u32>::from_stack(mem, stack_args + 8u32);
            let pdwInOut = <Option<&mut u32>>::from_stack(mem, stack_args + 12u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("dinput") {
                trace::Record::new(
                    dinput::IDirectInputDevice::GetDeviceData_pos,
                    "dinput",
                    "IDirectInputDevice::GetDeviceData",
                    &[
                        ("this", &this),
                        ("cbObjectData", &cbObjectData),
                        ("rgdod", &rgdod),
                        ("pdwInOut", &pdwInOut),
                        ("dwFlags", &dwFlags),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = dinput::IDirectInputDevice::GetDeviceData(
                sys,
                this,
                cbObjectData,
                rgdod,
                pdwInOut,
                dwFlags,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectInputDevice_SetCooperativeLevel(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let hwnd = <HWND>::from_stack(mem, stack_args + 4u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("dinput") {
                trace::Record::new(
                    dinput::IDirectInputDevice::SetCooperativeLevel_pos,
                    "dinput",
                    "IDirectInputDevice::SetCooperativeLevel",
                    &[("this", &this), ("hwnd", &hwnd), ("dwFlags", &dwFlags)],
                )
                .enter()
            } else {
                None
            };
            let result = dinput::IDirectInputDevice::SetCooperativeLevel(sys, this, hwnd, dwFlags);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectInputDevice_SetDataFormat(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpdf = <Option<&DIDATAFORMAT>>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("dinput") {
                trace::Record::new(
                    dinput::IDirectInputDevice::SetDataFormat_pos,
                    "dinput",
                    "IDirectInputDevice::SetDataFormat",
                    &[("this", &this), ("lpdf", &lpdf)],
                )
                .enter()
            } else {
                None
            };
            let result = dinput::IDirectInputDevice::SetDataFormat(sys, this, lpdf);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectInputDevice_SetEventNotification(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let hEvent = <u32>::from_stack(mem, stack_args + 4u32);
            let __trace_record = if trace::enabled("dinput") {
                trace::Record::new(
                    dinput::IDirectInputDevice::SetEventNotification_pos,
                    "dinput",
                    "IDirectInputDevice::SetEventNotification",
                    &[("this", &this), ("hEvent", &hEvent)],
                )
                .enter()
            } else {
                None
            };
            let result = dinput::IDirectInputDevice::SetEventNotification(sys, this, hEvent);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectInputDevice_SetProperty(
        sys: &mut dyn System,
        stack_args: u32,
    ) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let rguidProp = <Option<&GUID>>::from_stack(mem, stack_args + 4u32);
            let pdiph = <u32>::from_stack(mem, stack_args + 8u32);
            let __trace_record = if trace::enabled("dinput") {
                trace::Record::new(
                    dinput::IDirectInputDevice::SetProperty_pos,
                    "dinput",
                    "IDirectInputDevice::SetProperty",
                    &[
                        ("this", &this),
                        ("rguidProp", &rguidProp),
                        ("pdiph", &pdiph),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = dinput::IDirectInputDevice::SetProperty(sys, this, rguidProp, pdiph);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectInput_AddRef(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("dinput") {
                trace::Record::new(
                    dinput::IDirectInput::AddRef_pos,
                    "dinput",
                    "IDirectInput::AddRef",
                    &[("this", &this)],
                )
                .enter()
            } else {
                None
            };
            let result = dinput::IDirectInput::AddRef(sys, this);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectInput_CreateDevice(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let lpGUID = <Option<&GUID>>::from_stack(mem, stack_args + 4u32);
            let lplpDirectInputDevice = <Option<&mut u32>>::from_stack(mem, stack_args + 8u32);
            let pUnkOuter = <u32>::from_stack(mem, stack_args + 12u32);
            let __trace_record = if trace::enabled("dinput") {
                trace::Record::new(
                    dinput::IDirectInput::CreateDevice_pos,
                    "dinput",
                    "IDirectInput::CreateDevice",
                    &[
                        ("this", &this),
                        ("lpGUID", &lpGUID),
                        ("lplpDirectInputDevice", &lplpDirectInputDevice),
                        ("pUnkOuter", &pUnkOuter),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result = dinput::IDirectInput::CreateDevice(
                sys,
                this,
                lpGUID,
                lplpDirectInputDevice,
                pUnkOuter,
            );
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectInput_EnumDevices(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let dwDevType = <u32>::from_stack(mem, stack_args + 4u32);
            let callback = <u32>::from_stack(mem, stack_args + 8u32);
            let pvRef = <u32>::from_stack(mem, stack_args + 12u32);
            let dwFlags = <u32>::from_stack(mem, stack_args + 16u32);
            let __trace_record = if trace::enabled("dinput") {
                trace::Record::new(
                    dinput::IDirectInput::EnumDevices_pos,
                    "dinput",
                    "IDirectInput::EnumDevices",
                    &[
                        ("this", &this),
                        ("dwDevType", &dwDevType),
                        ("callback", &callback),
                        ("pvRef", &pvRef),
                        ("dwFlags", &dwFlags),
                    ],
                )
                .enter()
            } else {
                None
            };
            let result =
                dinput::IDirectInput::EnumDevices(sys, this, dwDevType, callback, pvRef, dwFlags);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
    pub unsafe fn IDirectInput_Release(sys: &mut dyn System, stack_args: u32) -> ABIReturn {
        unsafe {
            let mem = sys.mem().detach();
            let this = <u32>::from_stack(mem, stack_args + 0u32);
            let __trace_record = if trace::enabled("dinput") {
                trace::Record::new(
                    dinput::IDirectInput::Release_pos,
                    "dinput",
                    "IDirectInput::Release",
                    &[("this", &this)],
                )
                .enter()
            } else {
                None
            };
            let result = dinput::IDirectInput::Release(sys, this);
            if let Some(mut __trace_record) = __trace_record {
                __trace_record.exit(&result);
            }
            result.into()
        }
    }
}
const SHIMS: [Shim; 12usize] = [
    Shim {
        name: "DirectInputCreateA",
        func: Handler::Sync(wrappers::DirectInputCreateA),
    },
    Shim {
        name: "IDirectInputDevice::Acquire",
        func: Handler::Sync(wrappers::IDirectInputDevice_Acquire),
    },
    Shim {
        name: "IDirectInputDevice::EnumObjects",
        func: Handler::Sync(wrappers::IDirectInputDevice_EnumObjects),
    },
    Shim {
        name: "IDirectInputDevice::GetDeviceData",
        func: Handler::Sync(wrappers::IDirectInputDevice_GetDeviceData),
    },
    Shim {
        name: "IDirectInputDevice::SetCooperativeLevel",
        func: Handler::Sync(wrappers::IDirectInputDevice_SetCooperativeLevel),
    },
    Shim {
        name: "IDirectInputDevice::SetDataFormat",
        func: Handler::Sync(wrappers::IDirectInputDevice_SetDataFormat),
    },
    Shim {
        name: "IDirectInputDevice::SetEventNotification",
        func: Handler::Sync(wrappers::IDirectInputDevice_SetEventNotification),
    },
    Shim {
        name: "IDirectInputDevice::SetProperty",
        func: Handler::Sync(wrappers::IDirectInputDevice_SetProperty),
    },
    Shim {
        name: "IDirectInput::AddRef",
        func: Handler::Sync(wrappers::IDirectInput_AddRef),
    },
    Shim {
        name: "IDirectInput::CreateDevice",
        func: Handler::Sync(wrappers::IDirectInput_CreateDevice),
    },
    Shim {
        name: "IDirectInput::EnumDevices",
        func: Handler::Sync(wrappers::IDirectInput_EnumDevices),
    },
    Shim {
        name: "IDirectInput::Release",
        func: Handler::Sync(wrappers::IDirectInput_Release),
    },
];
pub const DLL: BuiltinDLL = BuiltinDLL {
    file_name: "dinput.dll",
    shims: &SHIMS,
    raw: std::include_bytes!("../dinput.dll"),
};
