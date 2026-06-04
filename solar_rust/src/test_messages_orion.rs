// Generated code!
#![allow(unused_comparisons, unreachable_patterns, unused_imports)]
#![allow(clippy::let_and_return, clippy::eq_op)]
#![allow(clippy::useless_conversion, clippy::unnecessary_cast)]
#![allow(
    clippy::excessive_precision,
    clippy::manual_range_contains,
    clippy::absurd_extreme_comparisons,
    clippy::too_many_arguments
)]
#![deny(clippy::arithmetic_side_effects)]

//! Message definitions from file `"Orion2_test_CANBUS.dbc"`
//!
//! - Version: `Version("")`

#[cfg(feature = "arb")]
use arbitrary::{Arbitrary, Unstructured};
use bitvec::prelude::*;
use core::ops::BitOr;
use embedded_can::{ExtendedId, Id, StandardId};

/// All messages
#[derive(Clone, Debug, defmt::Format)]
pub enum Messages {
    /// MSGID_0X380
    Msgid0x380(Msgid0x380),
    /// MSGID_0X522
    Msgid0x522(Msgid0x522),
    /// MSGID_0X37
    Msgid0x37(Msgid0x37),
    /// MSGID_0X3A
    Msgid0x3a(Msgid0x3a),
    /// MSGID_0X170
    Msgid0x170(Msgid0x170),
}

impl Messages {
    /// Read message from CAN frame
    #[inline(never)]
    pub fn from_can_message(id: Id, payload: &[u8]) -> Result<Self, CanError> {
        let res = match id {
            Msgid0x380::MESSAGE_ID => Messages::Msgid0x380(Msgid0x380::try_from(payload)?),
            Msgid0x522::MESSAGE_ID => Messages::Msgid0x522(Msgid0x522::try_from(payload)?),
            Msgid0x37::MESSAGE_ID => Messages::Msgid0x37(Msgid0x37::try_from(payload)?),
            Msgid0x3a::MESSAGE_ID => Messages::Msgid0x3a(Msgid0x3a::try_from(payload)?),
            Msgid0x170::MESSAGE_ID => Messages::Msgid0x170(Msgid0x170::try_from(payload)?),
            id => return Err(CanError::UnknownMessageId(id)),
        };
        Ok(res)
    }
}

/// MSGID_0X380
///
/// - Standard ID: 896 (0x380)
/// - Size: 8 bytes
/// - Transmitter: BMS
///
/// This ID Transmits at 8 ms.
#[derive(Clone, Copy)]
pub struct Msgid0x380 {
    raw: [u8; 8],
}

impl Msgid0x380 {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x380) });

    pub const PACK_CURRENT_MIN: f32 = 0_f32;
    pub const PACK_CURRENT_MAX: f32 = 0_f32;
    pub const PACK_OPEN_VOLTAGE_MIN: f32 = 0_f32;
    pub const PACK_OPEN_VOLTAGE_MAX: f32 = 0_f32;
    pub const PACK_SOC_MIN: f32 = 0_f32;
    pub const PACK_SOC_MAX: f32 = 0_f32;
    pub const RELAY_STATE_MIN: u16 = 0_u16;
    pub const RELAY_STATE_MAX: u16 = 0_u16;
    pub const CRC_CHECKSUM_MIN: u16 = 0_u16;
    pub const CRC_CHECKSUM_MAX: u16 = 0_u16;

    /// Construct new MSGID_0X380 from values
    pub fn new(
        pack_current: f32,
        pack_open_voltage: f32,
        pack_soc: f32,
        relay_state: u16,
        crc_checksum: u16,
    ) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_pack_current(pack_current)?;
        res.set_pack_open_voltage(pack_open_voltage)?;
        res.set_pack_soc(pack_soc)?;
        res.set_relay_state(relay_state)?;
        res.set_crc_checksum(crc_checksum)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// Pack_Current
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Amps"
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn pack_current(&self) -> f32 {
        self.pack_current_raw()
    }

    /// Get raw value of Pack_Current
    ///
    /// - Start bit: 7
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn pack_current_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[0..16].load_be::<u16>();

        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }

    /// Set value of Pack_Current
    #[inline(always)]
    pub fn set_pack_current(&mut self, value: f32) -> Result<(), CanError> {
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;

        self.raw.view_bits_mut::<Msb0>()[0..16].store_be(value);
        Ok(())
    }

    /// Pack_Open_Voltage
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Volts"
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn pack_open_voltage(&self) -> f32 {
        self.pack_open_voltage_raw()
    }

    /// Get raw value of Pack_Open_Voltage
    ///
    /// - Start bit: 23
    /// - Signal size: 16 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn pack_open_voltage_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[16..32].load_be::<u16>();

        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }

    /// Set value of Pack_Open_Voltage
    #[inline(always)]
    pub fn set_pack_open_voltage(&mut self, value: f32) -> Result<(), CanError> {
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;

        self.raw.view_bits_mut::<Msb0>()[16..32].store_be(value);
        Ok(())
    }

    /// Pack_SOC
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Percent"
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn pack_soc(&self) -> f32 {
        self.pack_soc_raw()
    }

    /// Get raw value of Pack_SOC
    ///
    /// - Start bit: 39
    /// - Signal size: 8 bits
    /// - Factor: 0.5
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn pack_soc_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[32..40].load_be::<u8>();

        let factor = 0.5_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }

    /// Set value of Pack_SOC
    #[inline(always)]
    pub fn set_pack_soc(&mut self, value: f32) -> Result<(), CanError> {
        let factor = 0.5_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[32..40].store_be(value);
        Ok(())
    }

    /// Relay_State
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn relay_state(&self) -> u16 {
        self.relay_state_raw()
    }

    /// Get raw value of Relay_State
    ///
    /// - Start bit: 47
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn relay_state_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Msb0>()[40..56].load_be::<u16>();

        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Relay_State
    #[inline(always)]
    pub fn set_relay_state(&mut self, value: u16) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x380::MESSAGE_ID,
        })?;
        let value = (value / factor) as u16;

        self.raw.view_bits_mut::<Msb0>()[40..56].store_be(value);
        Ok(())
    }

    /// CRC_Checksum
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn crc_checksum(&self) -> u16 {
        self.crc_checksum_raw()
    }

    /// Get raw value of CRC_Checksum
    ///
    /// - Start bit: 63
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 1720
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn crc_checksum_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Msb0>()[56..64].load_be::<u8>();

        let factor = 1;
        u16::from(signal)
            .saturating_mul(factor)
            .saturating_add(1720)
    }

    /// Set value of CRC_Checksum
    #[inline(always)]
    pub fn set_crc_checksum(&mut self, value: u16) -> Result<(), CanError> {
        let factor = 1;
        let value = value
            .checked_sub(1720)
            .ok_or(CanError::ParameterOutOfRange {
                message_id: Msgid0x380::MESSAGE_ID,
            })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[56..64].store_be(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for Msgid0x380 {
    type Error = CanError;

    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 {
            return Err(CanError::InvalidPayloadSize);
        }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Msgid0x380 {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
impl core::fmt::Debug for Msgid0x380 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Msgid0x380")
                .field("pack_current", &self.pack_current())
                .field("pack_open_voltage", &self.pack_open_voltage())
                .field("pack_soc", &self.pack_soc())
                .field("relay_state", &self.relay_state())
                .field("crc_checksum", &self.crc_checksum())
                .finish()
        } else {
            f.debug_tuple("Msgid0x380").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for Msgid0x380 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f,
            "Msgid0x380 {{ Pack_Current={:?} Pack_Open_Voltage={:?} Pack_SOC={:?} Relay_State={:?} CRC_Checksum={:?} }}",
            self.pack_current(),
            self.pack_open_voltage(),
            self.pack_soc(),
            self.relay_state(),
            self.crc_checksum(),
            );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Msgid0x380 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let pack_current = u.float_in_range(0_f32..=0_f32)?;
        let pack_open_voltage = u.float_in_range(0_f32..=0_f32)?;
        let pack_soc = u.float_in_range(0_f32..=0_f32)?;
        let relay_state = u.int_in_range(0..=0)?;
        let crc_checksum = u.int_in_range(0..=0)?;
        Msgid0x380::new(
            pack_current,
            pack_open_voltage,
            pack_soc,
            relay_state,
            crc_checksum,
        )
        .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// MSGID_0X522
///
/// - Standard ID: 1314 (0x522)
/// - Size: 8 bytes
/// - Transmitter: BMS
///
/// This ID Transmits at 104 ms.
#[derive(Clone, Copy)]
pub struct Msgid0x522 {
    raw: [u8; 8],
}

impl Msgid0x522 {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x522) });

    pub const FAILSAFE_STATUSES_MIN: u16 = 0_u16;
    pub const FAILSAFE_STATUSES_MAX: u16 = 0_u16;
    pub const HIGH_TEMPERATURE_MIN: u16 = 0_u16;
    pub const HIGH_TEMPERATURE_MAX: u16 = 0_u16;
    pub const AVERAGE_TEMPERATURE_MIN: u8 = 0_u8;
    pub const AVERAGE_TEMPERATURE_MAX: u8 = 0_u8;
    pub const BLANK_MIN: u8 = 0_u8;
    pub const BLANK_MAX: u8 = 0_u8;
    pub const BLANK_MIN: u8 = 0_u8;
    pub const BLANK_MAX: u8 = 0_u8;
    pub const CRC_CHECKSUM_MIN: u16 = 0_u16;
    pub const CRC_CHECKSUM_MAX: u16 = 0_u16;

    /// Construct new MSGID_0X522 from values
    pub fn new(
        failsafe_statuses: u16,
        high_temperature: u16,
        average_temperature: u8,
        blank: u8,
        blank: u8,
        crc_checksum: u16,
    ) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_failsafe_statuses(failsafe_statuses)?;
        res.set_high_temperature(high_temperature)?;
        res.set_average_temperature(average_temperature)?;
        res.set_blank(blank)?;
        res.set_blank(blank)?;
        res.set_crc_checksum(crc_checksum)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// Failsafe_Statuses
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn failsafe_statuses(&self) -> u16 {
        self.failsafe_statuses_raw()
    }

    /// Get raw value of Failsafe_Statuses
    ///
    /// - Start bit: 7
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn failsafe_statuses_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Msb0>()[0..16].load_be::<u16>();

        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Failsafe_Statuses
    #[inline(always)]
    pub fn set_failsafe_statuses(&mut self, value: u16) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x522::MESSAGE_ID,
        })?;
        let value = (value / factor) as u16;

        self.raw.view_bits_mut::<Msb0>()[0..16].store_be(value);
        Ok(())
    }

    /// High_Temperature
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Celsius"
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn high_temperature(&self) -> u16 {
        self.high_temperature_raw()
    }

    /// Get raw value of High_Temperature
    ///
    /// - Start bit: 23
    /// - Signal size: 16 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn high_temperature_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Msb0>()[16..32].load_be::<u16>();

        let factor = 1;
        u16::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of High_Temperature
    #[inline(always)]
    pub fn set_high_temperature(&mut self, value: u16) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x522::MESSAGE_ID,
        })?;
        let value = (value / factor) as u16;

        self.raw.view_bits_mut::<Msb0>()[16..32].store_be(value);
        Ok(())
    }

    /// Average_Temperature
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Celsius"
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn average_temperature(&self) -> u8 {
        self.average_temperature_raw()
    }

    /// Get raw value of Average_Temperature
    ///
    /// - Start bit: 39
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn average_temperature_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[32..40].load_be::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Average_Temperature
    #[inline(always)]
    pub fn set_average_temperature(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x522::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[32..40].store_be(value);
        Ok(())
    }

    /// Blank
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn blank(&self) -> u8 {
        self.blank_raw()
    }

    /// Get raw value of Blank
    ///
    /// - Start bit: 47
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn blank_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[40..48].load_be::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Blank
    #[inline(always)]
    pub fn set_blank(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x522::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[40..48].store_be(value);
        Ok(())
    }

    /// Blank
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn blank(&self) -> u8 {
        self.blank_raw()
    }

    /// Get raw value of Blank
    ///
    /// - Start bit: 55
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn blank_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[48..56].load_be::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Blank
    #[inline(always)]
    pub fn set_blank(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x522::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[48..56].store_be(value);
        Ok(())
    }

    /// CRC_Checksum
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn crc_checksum(&self) -> u16 {
        self.crc_checksum_raw()
    }

    /// Get raw value of CRC_Checksum
    ///
    /// - Start bit: 63
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 1721
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn crc_checksum_raw(&self) -> u16 {
        let signal = self.raw.view_bits::<Msb0>()[56..64].load_be::<u8>();

        let factor = 1;
        u16::from(signal)
            .saturating_mul(factor)
            .saturating_add(1721)
    }

    /// Set value of CRC_Checksum
    #[inline(always)]
    pub fn set_crc_checksum(&mut self, value: u16) -> Result<(), CanError> {
        let factor = 1;
        let value = value
            .checked_sub(1721)
            .ok_or(CanError::ParameterOutOfRange {
                message_id: Msgid0x522::MESSAGE_ID,
            })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[56..64].store_be(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for Msgid0x522 {
    type Error = CanError;

    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 {
            return Err(CanError::InvalidPayloadSize);
        }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Msgid0x522 {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
impl core::fmt::Debug for Msgid0x522 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Msgid0x522")
                .field("failsafe_statuses", &self.failsafe_statuses())
                .field("high_temperature", &self.high_temperature())
                .field("average_temperature", &self.average_temperature())
                .field("blank", &self.blank())
                .field("blank", &self.blank())
                .field("crc_checksum", &self.crc_checksum())
                .finish()
        } else {
            f.debug_tuple("Msgid0x522").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for Msgid0x522 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f,
            "Msgid0x522 {{ Failsafe_Statuses={:?} High_Temperature={:?} Average_Temperature={:?} Blank={:?} Blank={:?} CRC_Checksum={:?} }}",
            self.failsafe_statuses(),
            self.high_temperature(),
            self.average_temperature(),
            self.blank(),
            self.blank(),
            self.crc_checksum(),
            );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Msgid0x522 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let failsafe_statuses = u.int_in_range(0..=0)?;
        let high_temperature = u.int_in_range(0..=0)?;
        let average_temperature = u.int_in_range(0..=0)?;
        let blank = u.int_in_range(0..=0)?;
        let blank = u.int_in_range(0..=0)?;
        let crc_checksum = u.int_in_range(0..=0)?;
        Msgid0x522::new(
            failsafe_statuses,
            high_temperature,
            average_temperature,
            blank,
            blank,
            crc_checksum,
        )
        .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// MSGID_0X37
///
/// - Standard ID: 55 (0x37)
/// - Size: 8 bytes
/// - Transmitter: BMS
///
/// This ID Transmits at 8 ms.
#[derive(Clone, Copy)]
pub struct Msgid0x37 {
    raw: [u8; 8],
}

impl Msgid0x37 {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x37) });

    pub const HIGH_OPENCELL_VOLTAGE_MIN: f32 = 0_f32;
    pub const HIGH_OPENCELL_VOLTAGE_MAX: f32 = 0_f32;
    pub const AVG_OPENCELL_VOLTAGE_MIN: f32 = 0_f32;
    pub const AVG_OPENCELL_VOLTAGE_MAX: f32 = 0_f32;
    pub const BLANK_MIN: u8 = 0_u8;
    pub const BLANK_MAX: u8 = 0_u8;
    pub const PACK_INST_VOLTAGE_MIN: f32 = 0_f32;
    pub const PACK_INST_VOLTAGE_MAX: f32 = 0_f32;
    pub const BLANK_MIN: u8 = 0_u8;
    pub const BLANK_MAX: u8 = 0_u8;
    pub const BLANK_MIN: u8 = 0_u8;
    pub const BLANK_MAX: u8 = 0_u8;
    pub const BLANK_MIN: u8 = 0_u8;
    pub const BLANK_MAX: u8 = 0_u8;

    /// Construct new MSGID_0X37 from values
    pub fn new(
        high_opencell_voltage: f32,
        avg_opencell_voltage: f32,
        blank: u8,
        pack_inst_voltage: f32,
        blank: u8,
        blank: u8,
        blank: u8,
    ) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_high_opencell_voltage(high_opencell_voltage)?;
        res.set_avg_opencell_voltage(avg_opencell_voltage)?;
        res.set_blank(blank)?;
        res.set_pack_inst_voltage(pack_inst_voltage)?;
        res.set_blank(blank)?;
        res.set_blank(blank)?;
        res.set_blank(blank)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// High_Opencell_Voltage
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Volts"
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn high_opencell_voltage(&self) -> f32 {
        self.high_opencell_voltage_raw()
    }

    /// Get raw value of High_Opencell_Voltage
    ///
    /// - Start bit: 7
    /// - Signal size: 16 bits
    /// - Factor: 0.0001
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn high_opencell_voltage_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[0..16].load_be::<u16>();

        let factor = 0.0001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }

    /// Set value of High_Opencell_Voltage
    #[inline(always)]
    pub fn set_high_opencell_voltage(&mut self, value: f32) -> Result<(), CanError> {
        let factor = 0.0001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u16;

        self.raw.view_bits_mut::<Msb0>()[0..16].store_be(value);
        Ok(())
    }

    /// Avg_Opencell_Voltage
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Volts"
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn avg_opencell_voltage(&self) -> f32 {
        self.avg_opencell_voltage_raw()
    }

    /// Get raw value of Avg_Opencell_Voltage
    ///
    /// - Start bit: 23
    /// - Signal size: 8 bits
    /// - Factor: 0.0001
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn avg_opencell_voltage_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[16..24].load_be::<u8>();

        let factor = 0.0001_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }

    /// Set value of Avg_Opencell_Voltage
    #[inline(always)]
    pub fn set_avg_opencell_voltage(&mut self, value: f32) -> Result<(), CanError> {
        let factor = 0.0001_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[16..24].store_be(value);
        Ok(())
    }

    /// Blank
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn blank(&self) -> u8 {
        self.blank_raw()
    }

    /// Get raw value of Blank
    ///
    /// - Start bit: 31
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn blank_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[24..32].load_be::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Blank
    #[inline(always)]
    pub fn set_blank(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x37::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[24..32].store_be(value);
        Ok(())
    }

    /// Pack_Inst_Voltage
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Volts"
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn pack_inst_voltage(&self) -> f32 {
        self.pack_inst_voltage_raw()
    }

    /// Get raw value of Pack_Inst_Voltage
    ///
    /// - Start bit: 39
    /// - Signal size: 8 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn pack_inst_voltage_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[32..40].load_be::<u8>();

        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }

    /// Set value of Pack_Inst_Voltage
    #[inline(always)]
    pub fn set_pack_inst_voltage(&mut self, value: f32) -> Result<(), CanError> {
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[32..40].store_be(value);
        Ok(())
    }

    /// Blank
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn blank(&self) -> u8 {
        self.blank_raw()
    }

    /// Get raw value of Blank
    ///
    /// - Start bit: 47
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn blank_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[40..48].load_be::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Blank
    #[inline(always)]
    pub fn set_blank(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x37::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[40..48].store_be(value);
        Ok(())
    }

    /// Blank
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn blank(&self) -> u8 {
        self.blank_raw()
    }

    /// Get raw value of Blank
    ///
    /// - Start bit: 55
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn blank_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[48..56].load_be::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Blank
    #[inline(always)]
    pub fn set_blank(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x37::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[48..56].store_be(value);
        Ok(())
    }

    /// Blank
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn blank(&self) -> u8 {
        self.blank_raw()
    }

    /// Get raw value of Blank
    ///
    /// - Start bit: 63
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn blank_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[56..64].load_be::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Blank
    #[inline(always)]
    pub fn set_blank(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x37::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[56..64].store_be(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for Msgid0x37 {
    type Error = CanError;

    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 {
            return Err(CanError::InvalidPayloadSize);
        }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Msgid0x37 {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
impl core::fmt::Debug for Msgid0x37 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Msgid0x37")
                .field("high_opencell_voltage", &self.high_opencell_voltage())
                .field("avg_opencell_voltage", &self.avg_opencell_voltage())
                .field("blank", &self.blank())
                .field("pack_inst_voltage", &self.pack_inst_voltage())
                .field("blank", &self.blank())
                .field("blank", &self.blank())
                .field("blank", &self.blank())
                .finish()
        } else {
            f.debug_tuple("Msgid0x37").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for Msgid0x37 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f,
            "Msgid0x37 {{ High_Opencell_Voltage={:?} Avg_Opencell_Voltage={:?} Blank={:?} Pack_Inst_Voltage={:?} Blank={:?} Blank={:?} Blank={:?} }}",
            self.high_opencell_voltage(),
            self.avg_opencell_voltage(),
            self.blank(),
            self.pack_inst_voltage(),
            self.blank(),
            self.blank(),
            self.blank(),
            );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Msgid0x37 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let high_opencell_voltage = u.float_in_range(0_f32..=0_f32)?;
        let avg_opencell_voltage = u.float_in_range(0_f32..=0_f32)?;
        let blank = u.int_in_range(0..=0)?;
        let pack_inst_voltage = u.float_in_range(0_f32..=0_f32)?;
        let blank = u.int_in_range(0..=0)?;
        let blank = u.int_in_range(0..=0)?;
        let blank = u.int_in_range(0..=0)?;
        Msgid0x37::new(
            high_opencell_voltage,
            avg_opencell_voltage,
            blank,
            pack_inst_voltage,
            blank,
            blank,
            blank,
        )
        .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// MSGID_0X3A
///
/// - Standard ID: 58 (0x3a)
/// - Size: 8 bytes
/// - Transmitter: BMS
///
/// This ID Transmits at 8 ms.
#[derive(Clone, Copy)]
pub struct Msgid0x3a {
    raw: [u8; 8],
}

impl Msgid0x3a {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x3a) });

    pub const HIGH_CELL_RESISTANCE_MIN: f32 = 0_f32;
    pub const HIGH_CELL_RESISTANCE_MAX: f32 = 0_f32;
    pub const BLANK_MIN: u8 = 0_u8;
    pub const BLANK_MAX: u8 = 0_u8;
    pub const AVG_CELL_RESISTANCE_MIN: f32 = 0_f32;
    pub const AVG_CELL_RESISTANCE_MAX: f32 = 0_f32;
    pub const BLANK_MIN: u8 = 0_u8;
    pub const BLANK_MAX: u8 = 0_u8;
    pub const BLANK_MIN: u8 = 0_u8;
    pub const BLANK_MAX: u8 = 0_u8;
    pub const BLANK_MIN: u8 = 0_u8;
    pub const BLANK_MAX: u8 = 0_u8;
    pub const BLANK_MIN: u8 = 0_u8;
    pub const BLANK_MAX: u8 = 0_u8;
    pub const BLANK_MIN: u8 = 0_u8;
    pub const BLANK_MAX: u8 = 0_u8;

    /// Construct new MSGID_0X3A from values
    pub fn new(
        high_cell_resistance: f32,
        blank: u8,
        avg_cell_resistance: f32,
        blank: u8,
        blank: u8,
        blank: u8,
        blank: u8,
        blank: u8,
    ) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_high_cell_resistance(high_cell_resistance)?;
        res.set_blank(blank)?;
        res.set_avg_cell_resistance(avg_cell_resistance)?;
        res.set_blank(blank)?;
        res.set_blank(blank)?;
        res.set_blank(blank)?;
        res.set_blank(blank)?;
        res.set_blank(blank)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// High_Cell_Resistance
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mOhm"
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn high_cell_resistance(&self) -> f32 {
        self.high_cell_resistance_raw()
    }

    /// Get raw value of High_Cell_Resistance
    ///
    /// - Start bit: 7
    /// - Signal size: 8 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn high_cell_resistance_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[0..8].load_be::<u8>();

        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }

    /// Set value of High_Cell_Resistance
    #[inline(always)]
    pub fn set_high_cell_resistance(&mut self, value: f32) -> Result<(), CanError> {
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[0..8].store_be(value);
        Ok(())
    }

    /// Blank
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn blank(&self) -> u8 {
        self.blank_raw()
    }

    /// Get raw value of Blank
    ///
    /// - Start bit: 15
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn blank_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[8..16].load_be::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Blank
    #[inline(always)]
    pub fn set_blank(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x3a::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[8..16].store_be(value);
        Ok(())
    }

    /// Avg_Cell_Resistance
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "mOhm"
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn avg_cell_resistance(&self) -> f32 {
        self.avg_cell_resistance_raw()
    }

    /// Get raw value of Avg_Cell_Resistance
    ///
    /// - Start bit: 23
    /// - Signal size: 8 bits
    /// - Factor: 0.01
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn avg_cell_resistance_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[16..24].load_be::<u8>();

        let factor = 0.01_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }

    /// Set value of Avg_Cell_Resistance
    #[inline(always)]
    pub fn set_avg_cell_resistance(&mut self, value: f32) -> Result<(), CanError> {
        let factor = 0.01_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[16..24].store_be(value);
        Ok(())
    }

    /// Blank
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn blank(&self) -> u8 {
        self.blank_raw()
    }

    /// Get raw value of Blank
    ///
    /// - Start bit: 31
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn blank_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[24..32].load_be::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Blank
    #[inline(always)]
    pub fn set_blank(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x3a::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[24..32].store_be(value);
        Ok(())
    }

    /// Blank
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn blank(&self) -> u8 {
        self.blank_raw()
    }

    /// Get raw value of Blank
    ///
    /// - Start bit: 39
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn blank_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[32..40].load_be::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Blank
    #[inline(always)]
    pub fn set_blank(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x3a::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[32..40].store_be(value);
        Ok(())
    }

    /// Blank
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn blank(&self) -> u8 {
        self.blank_raw()
    }

    /// Get raw value of Blank
    ///
    /// - Start bit: 47
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn blank_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[40..48].load_be::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Blank
    #[inline(always)]
    pub fn set_blank(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x3a::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[40..48].store_be(value);
        Ok(())
    }

    /// Blank
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn blank(&self) -> u8 {
        self.blank_raw()
    }

    /// Get raw value of Blank
    ///
    /// - Start bit: 55
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn blank_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[48..56].load_be::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Blank
    #[inline(always)]
    pub fn set_blank(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x3a::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[48..56].store_be(value);
        Ok(())
    }

    /// Blank
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn blank(&self) -> u8 {
        self.blank_raw()
    }

    /// Get raw value of Blank
    ///
    /// - Start bit: 63
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn blank_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[56..64].load_be::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Blank
    #[inline(always)]
    pub fn set_blank(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x3a::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[56..64].store_be(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for Msgid0x3a {
    type Error = CanError;

    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 {
            return Err(CanError::InvalidPayloadSize);
        }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Msgid0x3a {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
impl core::fmt::Debug for Msgid0x3a {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Msgid0x3a")
                .field("high_cell_resistance", &self.high_cell_resistance())
                .field("blank", &self.blank())
                .field("avg_cell_resistance", &self.avg_cell_resistance())
                .field("blank", &self.blank())
                .field("blank", &self.blank())
                .field("blank", &self.blank())
                .field("blank", &self.blank())
                .field("blank", &self.blank())
                .finish()
        } else {
            f.debug_tuple("Msgid0x3a").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for Msgid0x3a {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f,
            "Msgid0x3a {{ High_Cell_Resistance={:?} Blank={:?} Avg_Cell_Resistance={:?} Blank={:?} Blank={:?} Blank={:?} Blank={:?} Blank={:?} }}",
            self.high_cell_resistance(),
            self.blank(),
            self.avg_cell_resistance(),
            self.blank(),
            self.blank(),
            self.blank(),
            self.blank(),
            self.blank(),
            );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Msgid0x3a {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let high_cell_resistance = u.float_in_range(0_f32..=0_f32)?;
        let blank = u.int_in_range(0..=0)?;
        let avg_cell_resistance = u.float_in_range(0_f32..=0_f32)?;
        let blank = u.int_in_range(0..=0)?;
        let blank = u.int_in_range(0..=0)?;
        let blank = u.int_in_range(0..=0)?;
        let blank = u.int_in_range(0..=0)?;
        let blank = u.int_in_range(0..=0)?;
        Msgid0x3a::new(
            high_cell_resistance,
            blank,
            avg_cell_resistance,
            blank,
            blank,
            blank,
            blank,
            blank,
        )
        .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// MSGID_0X170
///
/// - Standard ID: 368 (0x170)
/// - Size: 8 bytes
/// - Transmitter: BMS
///
/// This ID Transmits at 8 ms.
#[derive(Clone, Copy)]
pub struct Msgid0x170 {
    raw: [u8; 8],
}

impl Msgid0x170 {
    pub const MESSAGE_ID: embedded_can::Id =
        Id::Standard(unsafe { StandardId::new_unchecked(0x170) });

    pub const MAXIMUM_PACK_VOLTAGE_MIN: f32 = 0_f32;
    pub const MAXIMUM_PACK_VOLTAGE_MAX: f32 = 0_f32;
    pub const BLANK_MIN: u8 = 0_u8;
    pub const BLANK_MAX: u8 = 0_u8;
    pub const TOTAL_PACK_CYCLES_MIN: u8 = 0_u8;
    pub const TOTAL_PACK_CYCLES_MAX: u8 = 0_u8;
    pub const BLANK_MIN: u8 = 0_u8;
    pub const BLANK_MAX: u8 = 0_u8;
    pub const MAX_CELL_NUMBER_MIN: u8 = 0_u8;
    pub const MAX_CELL_NUMBER_MAX: u8 = 0_u8;
    pub const BLANK_MIN: u8 = 0_u8;
    pub const BLANK_MAX: u8 = 0_u8;
    pub const BLANK_MIN: u8 = 0_u8;
    pub const BLANK_MAX: u8 = 0_u8;
    pub const BLANK_MIN: u8 = 0_u8;
    pub const BLANK_MAX: u8 = 0_u8;

    /// Construct new MSGID_0X170 from values
    pub fn new(
        maximum_pack_voltage: f32,
        blank: u8,
        total_pack_cycles: u8,
        blank: u8,
        max_cell_number: u8,
        blank: u8,
        blank: u8,
        blank: u8,
    ) -> Result<Self, CanError> {
        let mut res = Self { raw: [0u8; 8] };
        res.set_maximum_pack_voltage(maximum_pack_voltage)?;
        res.set_blank(blank)?;
        res.set_total_pack_cycles(total_pack_cycles)?;
        res.set_blank(blank)?;
        res.set_max_cell_number(max_cell_number)?;
        res.set_blank(blank)?;
        res.set_blank(blank)?;
        res.set_blank(blank)?;
        Ok(res)
    }

    /// Access message payload raw value
    pub fn raw(&self) -> &[u8; 8] {
        &self.raw
    }

    /// Maximum_Pack_Voltage
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Volts"
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn maximum_pack_voltage(&self) -> f32 {
        self.maximum_pack_voltage_raw()
    }

    /// Get raw value of Maximum_Pack_Voltage
    ///
    /// - Start bit: 7
    /// - Signal size: 8 bits
    /// - Factor: 0.1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn maximum_pack_voltage_raw(&self) -> f32 {
        let signal = self.raw.view_bits::<Msb0>()[0..8].load_be::<u8>();

        let factor = 0.1_f32;
        let offset = 0_f32;
        (signal as f32) * factor + offset
    }

    /// Set value of Maximum_Pack_Voltage
    #[inline(always)]
    pub fn set_maximum_pack_voltage(&mut self, value: f32) -> Result<(), CanError> {
        let factor = 0.1_f32;
        let offset = 0_f32;
        let value = ((value - offset) / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[0..8].store_be(value);
        Ok(())
    }

    /// Blank
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn blank(&self) -> u8 {
        self.blank_raw()
    }

    /// Get raw value of Blank
    ///
    /// - Start bit: 15
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn blank_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[8..16].load_be::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Blank
    #[inline(always)]
    pub fn set_blank(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x170::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[8..16].store_be(value);
        Ok(())
    }

    /// Total_Pack_Cycles
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Num"
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn total_pack_cycles(&self) -> u8 {
        self.total_pack_cycles_raw()
    }

    /// Get raw value of Total_Pack_Cycles
    ///
    /// - Start bit: 23
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn total_pack_cycles_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[16..24].load_be::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Total_Pack_Cycles
    #[inline(always)]
    pub fn set_total_pack_cycles(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x170::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[16..24].store_be(value);
        Ok(())
    }

    /// Blank
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn blank(&self) -> u8 {
        self.blank_raw()
    }

    /// Get raw value of Blank
    ///
    /// - Start bit: 31
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn blank_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[24..32].load_be::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Blank
    #[inline(always)]
    pub fn set_blank(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x170::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[24..32].store_be(value);
        Ok(())
    }

    /// Max_Cell_Number
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: "Num"
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn max_cell_number(&self) -> u8 {
        self.max_cell_number_raw()
    }

    /// Get raw value of Max_Cell_Number
    ///
    /// - Start bit: 39
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn max_cell_number_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[32..40].load_be::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Max_Cell_Number
    #[inline(always)]
    pub fn set_max_cell_number(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x170::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[32..40].store_be(value);
        Ok(())
    }

    /// Blank
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn blank(&self) -> u8 {
        self.blank_raw()
    }

    /// Get raw value of Blank
    ///
    /// - Start bit: 47
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn blank_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[40..48].load_be::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Blank
    #[inline(always)]
    pub fn set_blank(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x170::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[40..48].store_be(value);
        Ok(())
    }

    /// Blank
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn blank(&self) -> u8 {
        self.blank_raw()
    }

    /// Get raw value of Blank
    ///
    /// - Start bit: 55
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn blank_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[48..56].load_be::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Blank
    #[inline(always)]
    pub fn set_blank(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x170::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[48..56].store_be(value);
        Ok(())
    }

    /// Blank
    ///
    /// - Min: 0
    /// - Max: 0
    /// - Unit: ""
    /// - Receivers: Third_Party_Device
    #[inline(always)]
    pub fn blank(&self) -> u8 {
        self.blank_raw()
    }

    /// Get raw value of Blank
    ///
    /// - Start bit: 63
    /// - Signal size: 8 bits
    /// - Factor: 1
    /// - Offset: 0
    /// - Byte order: BigEndian
    /// - Value type: Unsigned
    #[inline(always)]
    pub fn blank_raw(&self) -> u8 {
        let signal = self.raw.view_bits::<Msb0>()[56..64].load_be::<u8>();

        let factor = 1;
        u8::from(signal).saturating_mul(factor).saturating_add(0)
    }

    /// Set value of Blank
    #[inline(always)]
    pub fn set_blank(&mut self, value: u8) -> Result<(), CanError> {
        let factor = 1;
        let value = value.checked_sub(0).ok_or(CanError::ParameterOutOfRange {
            message_id: Msgid0x170::MESSAGE_ID,
        })?;
        let value = (value / factor) as u8;

        self.raw.view_bits_mut::<Msb0>()[56..64].store_be(value);
        Ok(())
    }
}

impl core::convert::TryFrom<&[u8]> for Msgid0x170 {
    type Error = CanError;

    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() != 8 {
            return Err(CanError::InvalidPayloadSize);
        }
        let mut raw = [0u8; 8];
        raw.copy_from_slice(&payload[..8]);
        Ok(Self { raw })
    }
}

impl embedded_can::Frame for Msgid0x170 {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        if id.into() != Self::MESSAGE_ID {
            None
        } else {
            data.try_into().ok()
        }
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        unimplemented!()
    }

    fn is_extended(&self) -> bool {
        match self.id() {
            Id::Standard(_) => false,
            Id::Extended(_) => true,
        }
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        Self::MESSAGE_ID
    }

    fn dlc(&self) -> usize {
        self.raw.len()
    }

    fn data(&self) -> &[u8] {
        &self.raw
    }
}
impl core::fmt::Debug for Msgid0x170 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if f.alternate() {
            f.debug_struct("Msgid0x170")
                .field("maximum_pack_voltage", &self.maximum_pack_voltage())
                .field("blank", &self.blank())
                .field("total_pack_cycles", &self.total_pack_cycles())
                .field("blank", &self.blank())
                .field("max_cell_number", &self.max_cell_number())
                .field("blank", &self.blank())
                .field("blank", &self.blank())
                .field("blank", &self.blank())
                .finish()
        } else {
            f.debug_tuple("Msgid0x170").field(&self.raw).finish()
        }
    }
}

impl defmt::Format for Msgid0x170 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f,
            "Msgid0x170 {{ Maximum_Pack_Voltage={:?} Blank={:?} Total_Pack_Cycles={:?} Blank={:?} Max_Cell_Number={:?} Blank={:?} Blank={:?} Blank={:?} }}",
            self.maximum_pack_voltage(),
            self.blank(),
            self.total_pack_cycles(),
            self.blank(),
            self.max_cell_number(),
            self.blank(),
            self.blank(),
            self.blank(),
            );
    }
}

#[cfg(feature = "arb")]
impl<'a> Arbitrary<'a> for Msgid0x170 {
    fn arbitrary(u: &mut Unstructured<'a>) -> Result<Self, arbitrary::Error> {
        let maximum_pack_voltage = u.float_in_range(0_f32..=0_f32)?;
        let blank = u.int_in_range(0..=0)?;
        let total_pack_cycles = u.int_in_range(0..=0)?;
        let blank = u.int_in_range(0..=0)?;
        let max_cell_number = u.int_in_range(0..=0)?;
        let blank = u.int_in_range(0..=0)?;
        let blank = u.int_in_range(0..=0)?;
        let blank = u.int_in_range(0..=0)?;
        Msgid0x170::new(
            maximum_pack_voltage,
            blank,
            total_pack_cycles,
            blank,
            max_cell_number,
            blank,
            blank,
            blank,
        )
        .map_err(|_| arbitrary::Error::IncorrectFormat)
    }
}

/// This is just to make testing easier
#[allow(dead_code)]
fn main() {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CanError {
    UnknownMessageId(embedded_can::Id),
    /// Signal parameter is not within the range
    /// defined in the dbc
    ParameterOutOfRange {
        /// dbc message id
        message_id: embedded_can::Id,
    },
    InvalidPayloadSize,
    /// Multiplexor value not defined in the dbc
    InvalidMultiplexor {
        /// dbc message id
        message_id: embedded_can::Id,
        /// Multiplexor value not defined in the dbc
        multiplexor: u16,
    },
}

impl core::fmt::Display for CanError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
#[cfg(feature = "std")]
impl std::error::Error for CanError {}
#[cfg(feature = "arb")]
trait UnstructuredFloatExt {
    fn float_in_range(&mut self, range: core::ops::RangeInclusive<f32>) -> arbitrary::Result<f32>;
}

#[cfg(feature = "arb")]
impl UnstructuredFloatExt for arbitrary::Unstructured<'_> {
    fn float_in_range(&mut self, range: core::ops::RangeInclusive<f32>) -> arbitrary::Result<f32> {
        let min = range.start();
        let max = range.end();
        let steps = u32::MAX;
        let factor = (max - min) / (steps as f32);
        let random_int: u32 = self.int_in_range(0..=steps)?;
        let random = min + factor * (random_int as f32);
        Ok(random)
    }
}
