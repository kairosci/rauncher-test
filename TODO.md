# TODO List - R Games Launcher

This document outlines future improvements and enhancements to make the launcher more robust and functional.

## High Priority - Robustness & Core Functionality

### Authentication & Security
- [x] **Token encryption at rest** - Store authentication tokens encrypted instead of plain JSON ✅
- [ ] **Automatic token refresh on expiry** - Refresh tokens transparently before they expire
- [x] **Rate limiting** - Implement exponential backoff for API requests to handle rate limits ✅
- [ ] **Session management** - Handle multiple concurrent sessions safely
- [x] **Secure credential storage** - File permissions (0600) on Unix systems ✅

### CDN & Download Infrastructure
- [x] **Real CDN manifest download** - Implement actual manifest file download from Epic CDN ✅
  - [x] Parse manifest URL from asset metadata ✅
  - [x] Handle gzip decompression ✅
  - [ ] Validate manifest signature
- [x] **Chunk download with retry logic** - Robust chunk downloading with automatic retries ✅
  - [x] Implement exponential backoff ✅
  - [ ] Handle partial downloads and resume
  - [x] Verify chunk integrity with SHA hashes ✅
- [ ] **Parallel downloads** - Multi-threaded chunk downloading for faster installations
  - [ ] Connection pooling
  - [ ] Configurable concurrent download limit
  - [ ] Bandwidth throttling option
- [x] **File reconstruction** - Assemble downloaded chunks into game files ✅
  - [ ] Handle sparse files correctly
  - [x] Set proper file permissions and attributes ✅
  - [x] Verify file integrity after reconstruction ✅

### Error Handling & Recovery
- [ ] **Installation resume capability** - Resume interrupted installations
  - [ ] Track downloaded chunks
  - [ ] Resume from last successful chunk
  - [ ] Verify partial installations before resuming
- [x] **Disk space checking** - Verify sufficient disk space before installation ✅
  - [x] Check available space against manifest build size ✅
  - [x] Reserve space for temporary files (10% buffer) ✅
  - [x] Warn user if space is insufficient ✅
- [x] **Network failure handling** - Graceful handling of network interruptions ✅
  - [x] Automatic retry with backoff ✅
  - [ ] Save partial progress
  - [ ] Resume downloads after network recovery
- [x] **Corrupt data detection** - Detect and handle corrupted downloads ✅
  - [x] Verify checksums for all downloaded data ✅
  - [x] Re-download corrupted chunks (via retry logic) ✅
  - [x] Validate complete installations ✅

### Progress Tracking & User Feedback
- [x] **Real-time progress reporting** - Detailed progress updates during operations ✅
  - [x] Download speed calculation ✅
  - [x] ETA estimation ✅
  - [x] Percentage completion ✅
  - [x] Current file/chunk being processed ✅
- [ ] **Progress persistence** - Save progress to disk for resume capability
- [ ] **GUI progress integration** - Connect CLI progress to GUI progress bars
- [ ] **Cancellation support** - Allow users to cancel long-running operations cleanly

## Medium Priority - Enhanced Functionality

### Game Management
- [x] **Differential updates** - Download only changed files for updates ✅
  - [x] Compare manifests to identify changes ✅
  - [x] Download only modified chunks ✅
  - [x] Reduce update download sizes ✅
- [x] **Installation verification** - Verify game installations are complete and valid ✅
  - [x] Compare installed files against manifest ✅
  - [x] Verify file checksums ✅
  - [x] Report missing or corrupted files ✅
- [ ] **Repair functionality** - Fix broken installations
  - [ ] Re-download corrupted files
  - [x] Fix file permissions ✅
  - [ ] Verify and repair installation metadata
- [ ] **DLC management** - Support for downloadable content
  - [ ] List available DLCs
  - [ ] Install/uninstall DLCs separately
  - [ ] Track DLC versions independently

### Cloud Saves
- [x] **Real cloud save API integration** - Connect to Epic's cloud save endpoints ✅
  - [x] Query available saves per game ✅
  - [x] Download save files ✅
  - [x] Upload save files ✅
- [x] **Save conflict resolution** - Handle conflicts between local and cloud saves ✅
  - [x] Compare timestamps ✅
  - [x] Show diff to user ✅
  - [x] Allow user to choose which version to keep ✅
- [x] **Automatic sync** - Background synchronization of saves ✅
  - [x] Sync on game launch/exit ✅
  - [x] Configurable sync frequency (via config.auto_update) ✅
  - [x] Conflict detection and resolution ✅
- [ ] **Save versioning** - Keep history of save states
  - [x] Store backup versions (*.backup) ✅
  - [ ] Allow rollback to previous saves
  - [ ] Automatic cleanup of old saves

### Performance & Optimization
- [x] **Caching layer** - Cache API responses and manifest data ✅
  - [ ] Cache game library
  - [x] Cache manifest metadata ✅
  - [x] Configurable cache TTL (1 hour) ✅
  - [x] Automatic cache invalidation ✅
- [ ] **Lazy loading** - Load data on-demand instead of all at once
- [ ] **Memory optimization** - Reduce memory footprint for large operations
  - Stream large files instead of loading entirely
  - Process chunks incrementally
  - Release resources proactively
- [ ] **Database backend** - Replace JSON files with SQLite for better performance
  - Store game metadata
  - Track installation state
  - Store download history

### Configuration & Settings
- [x] **Advanced configuration options** ✅
  - [x] Download speed limits (bandwidth_limit) ✅
  - [x] Concurrent download settings (download_threads) ✅
  - [x] CDN region selection ✅
  - [ ] Proxy support
- [x] **Configuration validation** - Validate config files on load ✅
- [x] **Configuration migration** - Handle config format changes gracefully ✅
- [ ] **Per-game settings** - Game-specific configuration overrides

## Low Priority - Nice to Have

### User Experience
- [ ] **Download queue** - Queue multiple games for installation
- [ ] **Installation scheduler** - Schedule installations for off-peak hours
- [ ] **Notifications** - Desktop notifications for completed operations
- [ ] **Search functionality** - Search games in library
- [ ] **Filters and sorting** - Advanced filtering and sorting options
- [ ] **Game categories** - Organize games into custom categories

### Analytics & Monitoring
- [ ] **Usage statistics** - Track launcher usage (with user consent)
- [ ] **Error reporting** - Optional anonymous error reporting
- [ ] **Performance metrics** - Track download speeds, installation times
- [ ] **Logging improvements**
  - Structured logging
  - Log rotation
  - Configurable log levels per module

### Integration & Compatibility
- [ ] **Steam integration** - Import games from Steam
- [ ] **Wine/Proton support** - Better Linux compatibility for Windows games
- [ ] **Controller support** - Gamepad navigation in GUI
- [ ] **Language localization** - Support for multiple languages
  - Italian translation
  - Spanish translation
  - German translation
  - French translation

### Developer Tools
- [ ] **Debug mode** - Enhanced debugging capabilities
  - Request/response logging
  - Mock API responses
  - Manifest inspection tools
- [ ] **Testing infrastructure**
  - Integration tests
  - Mock Epic API server for testing
  - Performance benchmarks
- [ ] **Documentation**
  - API documentation
  - Architecture diagrams
  - Contributing guide
  - Developer setup guide

## Security Considerations

### Critical Security TODOs
- [ ] **Input validation** - Validate all user inputs and API responses
- [ ] **Path traversal prevention** - Prevent directory traversal attacks in file operations
- [ ] **Dependency auditing** - Regular security audits of dependencies
- [ ] **Signed releases** - Sign release binaries for verification
- [ ] **Secure update mechanism** - Verify launcher updates are authentic
- [ ] **Sandboxing** - Sandbox game execution for security
- [ ] **Permission management** - Request only necessary permissions

## Code Quality & Maintenance

### Code Improvements
- [ ] **Error type refinement** - More specific error types for better handling
- [ ] **API client abstraction** - Abstract Epic API for easier testing
- [ ] **Dependency injection** - Better testability through DI
- [ ] **Code coverage** - Aim for >80% test coverage
- [ ] **Clippy warnings** - Address all Clippy suggestions
- [ ] **Documentation comments** - Add comprehensive doc comments

### Refactoring Opportunities
- [ ] **Separate concerns** - Better separation between API, business logic, UI
- [ ] **Trait-based design** - Use traits for extensibility
- [ ] **Async improvements** - Better async/await patterns
- [ ] **State management** - Centralized state management for GUI
- [ ] **Module organization** - Reorganize modules for clarity

## Platform-Specific

### Linux
- [ ] **Package managers** - Native packages for major distros (deb, rpm, AUR)
- [ ] **AppImage support** - Portable AppImage builds
- [ ] **Desktop integration** - Proper .desktop files and icons
- [ ] **System tray integration** - Background mode with system tray

### Cross-Platform (Future)
- [ ] **macOS support** - Port to macOS
- [ ] **BSD support** - Support for FreeBSD and other BSDs

---

## Implementation Notes

When implementing items from this TODO list:

1. **Prioritize robustness over features** - Stability and reliability first
2. **Test thoroughly** - Add tests for all new functionality
3. **Document decisions** - Comment why specific approaches were chosen
4. **Consider backwards compatibility** - Don't break existing functionality
5. **Performance test** - Profile performance-critical changes
6. **Security review** - Consider security implications of changes
7. **User experience** - Think about how changes affect users

## Contributing

If you want to contribute by implementing any of these TODOs:

1. Open an issue to discuss the implementation
2. Reference this TODO list in your PR
3. Update this list when you complete an item
4. Add tests for your implementation
5. Update documentation as needed
