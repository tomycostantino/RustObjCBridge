#import "NSWorkspaceWrapper.h"

bool openFile(const char* filePath) {
    @autoreleasepool {
        if (filePath == NULL) {
            return false;
        }

        NSString *path = [NSString stringWithUTF8String:filePath];
        NSURL *fileURL = [NSURL fileURLWithPath:path];

        return [[NSWorkspace sharedWorkspace] openURL:fileURL];
    }
}

bool openURL(const char* urlString) {
    @autoreleasepool {
        if (urlString == NULL) {
            return false;
        }

        NSString *urlStr = [NSString stringWithUTF8String:urlString];
        NSURL *url = [NSURL URLWithString:urlStr];

        if (url == nil) {
            return false;
        }

        return [[NSWorkspace sharedWorkspace] openURL:url];
    }
}

bool launchApplication(const char* bundleIdentifier) {
    @autoreleasepool {
        if (bundleIdentifier == NULL) {
            return false;
        }

        NSString *bundleID = [NSString stringWithUTF8String:bundleIdentifier];

        NSURL *appURL = [[NSWorkspace sharedWorkspace] URLForApplicationWithBundleIdentifier:bundleID];
        if (appURL == nil) {
            return false;
        }

        BOOL success = [[NSWorkspace sharedWorkspace] openURL:appURL];
        return success;
    }
}

const char* getApplicationPath(const char* bundleIdentifier) {
    @autoreleasepool {
        if (bundleIdentifier == NULL) {
            return NULL;
        }

        NSString *bundleID = [NSString stringWithUTF8String:bundleIdentifier];

        NSURL *appURL = [[NSWorkspace sharedWorkspace] URLForApplicationWithBundleIdentifier:bundleID];

        if (appURL == nil) {
            return NULL;
        }

        NSString *path = [appURL path];

        const char *result = strdup([path UTF8String]);
        return result;
    }
}

const char* getRunningApplications() {
    @autoreleasepool {
        NSArray<NSRunningApplication *> *runningApps = [[NSWorkspace sharedWorkspace] runningApplications];
        NSMutableArray *appInfoArray = [NSMutableArray arrayWithCapacity:runningApps.count];

        for (NSRunningApplication *app in runningApps) {
            NSDictionary *appInfo = @{
                @"bundleIdentifier": app.bundleIdentifier ?: @"",
                @"localizedName": app.localizedName ?: @"",
                @"executableURL": app.executableURL.path ?: @""
            };

            [appInfoArray addObject:appInfo];
        }

        NSError *error = nil;
        NSData *jsonData = [NSJSONSerialization dataWithJSONObject:appInfoArray options:0 error:&error];

        if (error || jsonData == nil) {
            return NULL;
        }

        NSString *jsonString = [[NSString alloc] initWithData:jsonData encoding:NSUTF8StringEncoding];
        const char *result = strdup([jsonString UTF8String]);
        return result;
    }
}

bool hideApplication(const char* bundleIdentifier) {
    @autoreleasepool {
        if (bundleIdentifier == NULL) {
            return false;
        }

        NSString *bundleID = [NSString stringWithUTF8String:bundleIdentifier];
        NSArray<NSRunningApplication *> *apps = [NSRunningApplication runningApplicationsWithBundleIdentifier:bundleID];

        if (apps.count == 0) {
            return false;
        }

        return [apps.firstObject hide];
    }
}

bool unhideApplication(const char* bundleIdentifier) {
    @autoreleasepool {
        if (bundleIdentifier == NULL) {
            return false;
        }

        NSString *bundleID = [NSString stringWithUTF8String:bundleIdentifier];
        NSArray<NSRunningApplication *> *apps = [NSRunningApplication runningApplicationsWithBundleIdentifier:bundleID];

        if (apps.count == 0) {
            return false;
        }

        return [apps.firstObject unhide];
    }
}

const char* getFrontmostApplication() {
    @autoreleasepool {
        NSRunningApplication *frontmostApp = [[NSWorkspace sharedWorkspace] frontmostApplication];

        if (frontmostApp == nil) {
            return NULL;
        }

        NSDictionary *appInfo = @{
            @"bundleIdentifier": frontmostApp.bundleIdentifier ?: @"",
            @"localizedName": frontmostApp.localizedName ?: @"",
            @"executableURL": frontmostApp.executableURL.path ?: @"",
            @"processIdentifier": @(frontmostApp.processIdentifier),
            @"launchDate": frontmostApp.launchDate ? [NSString stringWithFormat:@"%@", frontmostApp.launchDate] : @""
        };

        NSError *error = nil;
        NSData *jsonData = [NSJSONSerialization dataWithJSONObject:appInfo options:0 error:&error];

        if (error || jsonData == nil) {
            return NULL;
        }

        NSString *jsonString = [[NSString alloc] initWithData:jsonData encoding:NSUTF8StringEncoding];
        const char *result = strdup([jsonString UTF8String]);
        return result;
    }
}

void freeString(const char* str) {
    if (str != NULL) {
        free((void*)str);
    }
}