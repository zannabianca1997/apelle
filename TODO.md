# Features

- [x] Youtube player
- [x] Sync the music from the player to the backend (slow loading, ads or other)
- [x] Autostart of the next song
- [x] Buttons to control autoplay, player, navigation (nav bar)
- [ ] Change the party name
- [x] Volume selector
- [ ] Progress bar
- [x] Buttons to delete/ban queued song and insta-start it
- [x] Search functionality
- [ ] Add multiple songs/show if a song has already been added
- [x] Autolike: songs that are added will be automatically liked
- [x] Move buttons under a context menu
- [ ] Full configuration at queue creation
- [ ] Change other user type (make admin)
- [ ] Private queue (no default role + invite links)

# Possible future expansions

- [ ] Origin for the youtube api
- [ ] Use the playlist feature of the player to enable preloading
- [ ] Mobile interface

# Chores

- [x] Move all configurations to the database and enable per-queue configuration
- [x] Complete the i18n for italian

# Bugs

- [x] Sometimes the video after a stop restart from a previous point
- [x] Adding musics/making any change make the player jumps erratically
- [x] Player is not synced when manually started
- [x] CI runs even if the changes are non-functional (e.g. changes in the README.md or TODO.md)
- [x] Autolike not counted in song ordering
- [x] When the tab is not active, the song is repeatedly synced to the wrong position
