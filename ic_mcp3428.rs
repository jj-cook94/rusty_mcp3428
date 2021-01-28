//addr
//This function takes no input.
//This function returns the I2C address of the MCP3428 ADC IC

pub fn addr() -> u8 {
	let addr = 0b01101000;
	return addr;
}


//cmd_read
//This function takes arguments correcponding to the channel selection, sample rate and converter gain.
//This function returns a binary opcode that can be passed over I2C that will cause the ADC to convert
//a reading with the selected settings

pub fn cmd_read(CSB: u8, SRSB: u8, PGAGSB: u8) -> u8 {
	//start with the oneshot read active - if this board function is called it is assumed that a measurement is desired
	let mut cmd = 0b10000000;

	// Channel Selection bit 
	match CSB {
		0 => (cmd |= 0b00000000), //zero case does nothing in each instance
		1 => (cmd |= 0b00000000), //Channel A
		2 => (cmd |= 0b00100000), //Channel B
		3 => (cmd |= 0b01000000), //Channel C
		4 => (cmd |= 0b01100000), //Channel D
		_ => (cmd |= 0b00000000), //other cases also do nothing
	};

	//Sample Rate Selection bit
	match SRSB {
		0 => (cmd |= 0b00000000), //12 Bit
		1 => (cmd |= 0b00000000), //12 Bit
		2 => (cmd |= 0b00000100), //14 Bit
		3 => (cmd |= 0b00001000), //16 Bit
		_ => (cmd |= 0b00000000), //12 Bit
	};

	//Programmable Gain Amplifier Gain Selection bit
	match PGAGSB {
		0 => (cmd |= 0b00000000), //1x Gain
		1 => (cmd |= 0b00000000), //1x Gain
		2 => (cmd |= 0b00000001), //2x Gain
		3 => (cmd |= 0b00000010), //4x Gain
		4 => (cmd |= 0b00000011), //8x Gain
		_ => (cmd |= 0b00000000), //1x Gain
	};

	return cmd;
}