# TODO List - R Games Launcher

This document outlines future improvements and enhancements to make the launcher more robust and functional.

## High Priority - Robustness & Core Functionality

### Authentication & Security
- [ ] **Token encryption at rest** - Store authentication tokens encrypted instead of plain JSON
- [ ] **Automatic token refresh on expiry** - Refresh tokens transparently before they expire
- [ ] **Rate limiting** - Implement exponential backoff for API requests to handle rate limits
- [ ] **Session management** - Handle multiple concurrent sessions safely
- [ ] **Secure credential storage** - Use OS keychain/credential manager instead of files

### CDN & Download Infrastructure
- [ ] **Real CDN manifest download** - Implement actual manifest file download from Epic CDN
  - Parse manifest URL from asset metadata
  - Handle gzip decompression
  - Validate manifest signature
- [ ] **Chunk download with retry logic** - Robust chunk downloading with automatic retries
  - Implement exponential backoff
  - Handle partial downloads and resume
  - Verify chunk integrity with SHA hashes
- [ ] **Parallel downloads** - Multi-threaded chunk downloading for faster installations
  - Connection pooling
  - Configurable concurrent download limit
  - Bandwidth throttling option
- [ ] **File reconstruction** - Assemble downloaded chunks into game files
  - Handle sparse files correctly
  - Set proper file permissions and attributes
  - Verify file integrity after reconstruction

### Error Handling & Recovery
- [ ] **Installation resume capability** - Resume interrupted installations
  - Track downloaded chunks
  - Resume from last successful chunk
  - Verify partial installations before resuming
- [ ] **Disk space checking** - Verify sufficient disk space before installation
  - Check available space against manifest build size
  - Reserve space for temporary files
  - Warn user if space is insufficient
- [ ] **Network failure handling** - Graceful handling of network interruptions
  - Automatic retry with backoff
  - Save partial progress
  - Resume downloads after network recovery
- [ ] **Corrupt data detection** - Detect and handle corrupted downloads
  - Verify checksums for all downloaded data
  - Re-download corrupted chunks
  - Validate complete installations

### Progress Tracking & User Feedback
- [ ] **Real-time progress reporting** - Detailed progress updates during operations
  - Download speed calculation
  - ETA estimation
  - Percentage completion
  - Current file/chunk being processed
- [ ] **Progress persistence** - Save progress to disk for resume capability
- [ ] **GUI progress integration** - Connect CLI progress to GUI progress bars
- [ ] **Cancellation support** - Allow users to cancel long-running operations cleanly

## Medium Priority - Enhanced Functionality

### Game Management
- [ ] **Differential updates** - Download only changed files for updates
  - Compare manifests to identify changes
  - Download only modified chunks
  - Reduce update download sizes
- [ ] **Installation verification** - Verify game installations are complete and valid
  - Compare installed files against manifest
  - Verify file checksums
  - Report missing or corrupted files
- [ ] **Repair functionality** - Fix broken installations
  - Re-download corrupted files
  - Fix file permissions
  - Verify and repair installation metadata
- [ ] **DLC management** - Support for downloadable content
  - List available DLCs
  - Install/uninstall DLCs separately
  - Track DLC versions independently

### Cloud Saves
- [ ] **Real cloud save API integration** - Connect to Epic's cloud save endpoints
  - Query available saves per game
  - Download save files
  - Upload save files
- [ ] **Save conflict resolution** - Handle conflicts between local and cloud saves
  - Compare timestamps
  - Show diff to user
  - Allow user to choose which version to keep
- [ ] **Automatic sync** - Background synchronization of saves
  - Sync on game launch/exit
  - Configurable sync frequency
  - Conflict detection and resolution
- [ ] **Save versioning** - Keep history of save states
  - Store multiple save versions
  - Allow rollback to previous saves
  - Automatic cleanup of old saves

### Performance & Optimization
- [ ] **Caching layer** - Cache API responses and manifest data
  - Cache game library
  - Cache manifest metadata
  - Configurable cache TTL
  - Automatic cache invalidation
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
- [ ] **Advanced configuration options**
  - Download speed limits
  - Concurrent download settings
  - CDN region selection
  - Proxy support
- [ ] **Configuration validation** - Validate config files on load
- [ ] **Configuration migration** - Handle config format changes gracefully
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
