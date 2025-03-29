#import <Foundation/Foundation.h>
#import <AppKit/AppKit.h>

#ifdef __cplusplus
extern "C" {
#endif

bool openFile(const char* filePath);

bool openURL(const char* urlString);

bool launchApplication(const char* bundleIdentifier);

const char* getApplicationPath(const char* bundleIdentifier);

const char* getRunningApplications();

bool hideApplication(const char* bundleIdentifier);

bool unhideApplication(const char* bundleIdentifier);

const char* getFrontmostApplication();

void freeString(const char* str);

#ifdef __cplusplus
}
#endif