//
// sprocketnes/main.rs
//
// Copyright (c) 2012 Mozilla Foundation
// Author: Patrick Walton
//

use cpu::Cpu;
use gfx::Gfx;
use mapper::Mapper;
use mem::MemMap;
use ppu::{Oam, Ppu, Vram};
use rom::Rom;
use util::println;

use core::cast::transmute;
use core::libc::size_t;
use core::task::PlatformThread;
use core::{libc, os, str};
use sdl;

fn start() {
    let args = os::args();
    if args.len() < 2 {
        println("usage: sprocketnes <path-to-rom>");
        return;
    }

    let rom = Rom::from_path(args[1]);
    println("Loaded ROM:");
    println(rom.header.to_str());

    let gfx = Gfx::new();
    let mut mapper = Mapper::new(&rom);
    let mut ppu = Ppu::new(Vram::new(&rom), Oam::new());
    let mut memmap = MemMap::new(ppu, mapper);
    let mut cpu = Cpu::new(memmap);

    // TODO: For testing purposes (nestest.log)...
    // cpu.reset();

    for 1000.times {
        cpu.step();
    }

    do gfx.screen.with_lock |pixels| {
        for vec::each_mut(pixels) |pixel| {
            *pixel = 0x80;
        }
    }

    gfx.screen.flip();

    loop {
        match sdl::event::poll_event() {
            sdl::event::KeyUpEvent(*) => break,
            _ => {}
        }
    }
}

fn main() {
    sdl::start::start(start);
}

