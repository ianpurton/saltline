Implement a high-fidelity animated splash / main menu screen for the existing Rust + Bevy 0.20 Saltline project.

This is an existing project. Do not start a new game and do not rewrite the gameplay. Inspect the current repository first.

Use `assets/reference/splash-screen.png` as the primary visual reference.

## Goal

Reproduce the original Saltline title screen as closely as practical in Bevy.

The important detail is that the splash screen is **not a static image**. It uses a **live animated 3D dockyard background** with a slow cinematic camera move through the port while the menu UI overlays the scene.

## Requirements

### 1. Menu state
- Add or refine a dedicated title / splash screen state.
- This state should appear before gameplay.
- It must support:
  - Deploy
  - Settings
  - Controls
- Deploy should start the game.
- Esc or appropriate input should back out of subpanels if needed.

### 2. Animated 3D background
- Use the actual procedural dockyard environment as the background, not a flat image.
- Create a cinematic title-screen camera path moving slowly through the port.
- The motion should feel like a gentle tracked/dolly shot, similar to the original.
- The camera should loop smoothly or reset cleanly.
- Favor a composition similar to the reference:
  - shipping containers framing the scene
  - large crane structure in the distance
  - wet ground and reflections
  - moody dawn / dusk backlighting
  - rain and fog visible
- Keep the background visually alive even while idle:
  - rain streaks / particles
  - subtle atmospheric motion
  - slight light flicker or environmental motion if useful
- Do not spawn active enemies or distracting gameplay action in the menu background unless it clearly improves the scene.

### 3. UI overlay
Match the reference layout and tone as closely as practical.

Implement:
- Large “SALTLINE” title
- Small header text above it
- Mission summary text:
  - “The port’s security net has gone feral. Cut its three uplinks. Reach the boat.”
- Buttons:
  - DEPLOY
  - SETTINGS
  - CONTROLS
- Control legend on the lower-right:
  - WASD move
  - mouse look
  - LMB fire
  - RMB aim
  - R reload
  - E cut relay
  - Space jump
  - Shift sprint
  - C crouch
  - Esc pause
- Small footer text similar in spirit to the reference

Use Bevy UI and typography to create a close visual match:
- large bold condensed title
- smaller technical uppercase labels
- yellow/orange accent color for highlighted elements
- translucent dark panels behind text where needed
- clean alignment and spacing

### 4. Visual quality
Improve the menu presentation so it feels polished and cinematic:
- strong silhouette and framing
- wet reflective ground
- atmospheric fog
- rain
- backlit industrial scene
- high contrast without losing readability
- readable UI over the moving scene

Use existing Bevy rendering features already in the project.
Do not rewrite the renderer.

### 5. Implementation guidance
- Reuse the existing world generation where practical.
- Create a dedicated menu camera and menu scene setup.
- If needed, create a lighter-weight menu scene variant focused on composition rather than gameplay.
- Keep the title-screen implementation modular.
- Keep gameplay logic separate from menu presentation.
- Do not break existing tests or gameplay flow.

### 6. Verification
- Run the game and verify the splash screen appears first.
- Capture screenshots of the title screen.
- Compare the result to `assets/reference/splash-screen.png`.
- Refine layout, camera framing, lighting and readability if substantial differences remain.
- Do not stop at the first functional menu if it still looks significantly worse than the reference.

### 7. Completion criteria
The task is complete when:
- the game starts on a polished splash screen
- the background is a moving cinematic dockyard scene
- the UI closely matches the reference layout and style
- Deploy starts gameplay correctly
- the menu presentation feels substantially closer to the original Saltline title screen than the current version
