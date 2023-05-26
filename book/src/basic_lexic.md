# Logic vs Render spawning.

spawn_x => Logic bundle.  
spawn_x_render => Sprite bundle with visibility and transform and the like.  
The render systems work from logic systems, but the logic systems don't know anything about them.  
Only the logic components will be saved and loaded, the render ones will be reconstruct on Startup time.  