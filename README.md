An old Bevy v0.12 Vampire Surivors clone I tried doing in a few weeks for my sibling's wedding. Was a mad rush so the code is a nightmare, but it works, with XP bugs and bad damage-modifier handling.

NB TO RUN THIS:

- This uses an old version of Bevy and Rust from 2024/25. Please ensure you're using the expected toolchain by running ```rustup default 1.84.1```.
- On Linux, ensure that you update the vendor file /home/%user%/.cargo/registry/src/index.crates.io-6f17d22bba15001f/bevy_render-0.12.1/src/view/window/mod.rs.rs in the bevy source files to have the code below. There's a bug in the older code due to wayland changes on Linux, if you have crashes running it. The path will vary potentially, but window/mod.rs in the bevy render crate under 0.12.1 should be the one:

```rust
        let not_already_configured = window_surfaces.configured_windows.insert(window.entity);

        let surface = &surface_data.surface;
        if not_already_configured || window.size_changed || window.present_mode_changed {
            render_device.configure_surface(surface, &surface_configuration);
            // let frame = surface
            //     .get_current_texture()
            //     .expect("Error configuring surface");
            // window.set_swapchain_texture(frame);
        }

        match surface.get_current_texture() {
            Ok(frame) => {
                window.set_swapchain_texture(frame);
            }
            Err(wgpu::SurfaceError::Outdated) => {
                // Linux Nvidia 550 driver often returns outdated when resizing the window
                continue;
            }
            #[cfg(target_os = "linux")]
            Err(wgpu::SurfaceError::Timeout) if may_erroneously_timeout() => {
                bevy_utils::tracing::trace!(
                    "Couldn't get swap chain texture. This is probably a quirk \
                        of your Linux GPU driver, so it can be safely ignored."
                );
            }
            Err(err) => {
                panic!("Couldn't get swap chain texture, operation unrecoverable: {err}");
            }
        }
```

- Copy the assets folder into the same folder as the release binary (./target/release) for it to pick them up correctly. I still have to figure out what's happening here.
- Run the binary (./target/release/wed) and it should work, if you're on a similar platform to myself at least.
