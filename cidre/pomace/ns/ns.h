//
//  ns.h
//  ns
//
//  Created by Yury Korolev on 07.07.2022.
//

#import <Foundation/Foundation.h>
#include "../macro.h"

NS_ASSUME_NONNULL_BEGIN

@interface CidreMachPortDelegate : NSObject<NSMachPortDelegate> {
  @public void * _vtable[2];
}
@end

NS_RETURNS_RETAINED
CidreMachPortDelegate * make_mach_port_delegate(void * _Nonnull vtable[_Nonnull 2]) {
  CidreMachPortDelegate * result = [CidreMachPortDelegate new];
  memcpy(result->_vtable, vtable, 2 * sizeof(void *));
  return result;
}

void cidre_raise_exception(NSString *message) {
  [NSException raise:NSGenericException format:@"%@", message];
}

void cidre_throw_exception(NSString *message) {
  @throw message;
}

id _Nullable cidre_try_catch(void (*during)(void *), void * context ) {
  @try {
    during(context);
    return nil;
  } @catch (id e) {
    return e;
  }
}

#pragma mark - NSURLSession

NS_RETURNS_RETAINED
rsel1(, id, dataTaskWithURL, NSURL *, NSURLSessionDataTask *)
NS_RETURNS_RETAINED
rsel1(, id, dataTaskWithRequest, NSURLRequest *, NSURLSessionDataTask *)

#pragma mark - NSURLSessionTask

wsel0(NSURLSessionTask_, NSURLSessionTask *, resume)
wsel0(NSURLSessionTask_, NSURLSessionTask *, cancel)
wsel0(NSURLSessionTask_, NSURLSessionTask *, suspend)
rsel0(NSURLSessionTask_, NSURLSessionTask *, state, NSURLSessionTaskState)
rsel0(NSURLSessionTask_, NSURLSessionTask *, error, NSError *)

rsel0(NSURLSessionTask_, NSURLSessionTask *, taskIdentifier, NSUInteger)
rsel0(NSURLSessionTask_, NSURLSessionTask *, originalRequest, NSURLRequest * _Nullable)
rsel0(NSURLSessionTask_, NSURLSessionTask *, currentRequest, NSURLRequest * _Nullable)
rsel0(NSURLSessionTask_, NSURLSessionTask *, response, NSURLResponse * _Nullable)

rwsel(NSURLSessionTask_, NSURLSessionTask *, priority, setPriority, float)

#pragma mark - NSURLRequest

NS_RETURNS_RETAINED
csel1(, NSURLRequest, requestWithURL, NSURL *, NSURLRequest *)

NS_RETURNS_RETAINED
csel3(, NSURLRequest, requestWithURL, NSURL *, cachePolicy, NSURLRequestCachePolicy, timeoutInterval, NSTimeInterval, NSURLRequest *)

rsel0(NSURLRequest_, NSURLRequest *, cachePolicy, NSURLRequestCachePolicy)
rsel0(NSURLRequest_, NSURLRequest *, timeoutInterval, NSTimeInterval)
rsel0(NSURLRequest_, NSURLRequest *, networkServiceType, NSURLRequestNetworkServiceType)
rsel0(NSURLRequest_, NSURLRequest *, allowsCellularAccess, BOOL)
rsel0(NSURLRequest_, NSURLRequest *, allowsExpensiveNetworkAccess, BOOL)
rsel0(NSURLRequest_, NSURLRequest *, allowsConstrainedNetworkAccess, BOOL)
rsel0(NSURLRequest_, NSURLRequest *, assumesHTTP3Capable, BOOL)
rsel0(NSURLRequest_, NSURLRequest *, attribution, NSURLRequestAttribution)
rsel0(NSURLRequest_, NSURLRequest *, requiresDNSSECValidation, BOOL)

rsel0(NSURLRequest_, NSURLRequest *, HTTPMethod, NSString *)
rsel0(NSURLRequest_, NSURLRequest *, allHTTPHeaderFields, NSDictionary * _Nullable)

rsel1(NSURLRequest_, NSURLRequest *, valueForHTTPHeaderField, NSString *, NSString * _Nullable)
rsel0(NSURLRequest_, NSURLRequest *, HTTPBody, NSData * _Nullable)

#pragma mark - NSMutableURLRequest

NS_RETURNS_RETAINED
csel1(, NSMutableURLRequest, requestWithURL, NSURL *, NSURLRequest *)

NS_RETURNS_RETAINED
csel3(, NSMutableURLRequest, requestWithURL, NSURL *, cachePolicy, NSURLRequestCachePolicy, timeoutInterval, NSTimeInterval, NSURLRequest *)

rsel0(NSMutableURLRequest_, NSMutableURLRequest *, cachePolicy, NSURLRequestCachePolicy)

wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setCachePolicy, NSURLRequestCachePolicy)
wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setTimeoutInterval, NSTimeInterval)
wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setNetworkServiceType, NSURLRequestNetworkServiceType)
wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setAllowsCellularAccess, BOOL)
wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setAllowsExpensiveNetworkAccess, BOOL)
wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setAllowsConstrainedNetworkAccess, BOOL)
wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setAssumesHTTP3Capable, BOOL)
wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setAttribution, NSURLRequestAttribution)
wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setRequiresDNSSECValidation, BOOL)

wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setHTTPMethod, NSString * _Nullable)
wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setAllHTTPHeaderFields, NSDictionary * _Nullable)
wsel1(NSMutableURLRequest_, NSMutableURLRequest *, setHTTPBody, NSData * _Nullable)


#pragma mark NSURLResponse

NS_RETURNS_RETAINED
asel4(, NSURLResponse, initWithURL, NSURL *, MIMEType, NSString *, expectedContentLength, NSInteger, textEncodingName, NSString *)

#pragma mark NSURLSessionWebSocketMessage

asel1(, NSURLSessionWebSocketMessage, initWithData, NSData *)
asel1(, NSURLSessionWebSocketMessage, initWithString, NSString *)

rsel0(NSURLSessionWebSocketMessage_, NSURLSessionWebSocketMessage *, type, NSURLSessionWebSocketMessageType)
rsel0(NSURLSessionWebSocketMessage_, NSURLSessionWebSocketMessage *, data, NSData * _Nullable)
rsel0(NSURLSessionWebSocketMessage_, NSURLSessionWebSocketMessage *, string, NSString * _Nullable)

#pragma mark NSURLCache

csel0(, NSURLCache, sharedURLCache, NSURLCache *)

asel3(, NSURLCache, initWithMemoryCapacity, NSUInteger, diskCapacity, NSUInteger, directoryURL, NSURL * _Nullable)

#pragma mark NSData

NS_RETURNS_RETAINED
csel3(, NSData, dataWithContentsOfFile, NSString *, options, NSDataReadingOptions, error, NSError **, NSData *)

NS_RETURNS_RETAINED
csel3(, NSData, dataWithContentsOfURL, NSURL *, options, NSDataReadingOptions, error, NSError **, NSData *)

rsel2(, id, writeToFile,NSString *,atomically, BOOL, BOOL)

#pragma mark - NSNumber

NS_RETURNS_RETAINED
csel1(, NSNumber, numberWithInteger, NSInteger, NSNumber *)

#pragma mark - NSRegularExpression

NS_RETURNS_RETAINED
csel3(, NSRegularExpression, regularExpressionWithPattern, NSString *, options, NSRegularExpressionOptions, error, NSError **, NSRegularExpression *)


Class NS_NUMBER;
Class NS_ARRAY;
Class NS_MUTABLE_ARRAY;
Class NS_STRING;
Class NS_MUTABLE_STRING;
Class NS_SET;
Class NS_MUTABLE_SET;
Class NS_URL;
Class NS_DATA;
Class NS_MUTABLE_DATA;
Class NS_PROCESS_INFO;
Class NS_URL_SESSION;
Class NS_URL_CACHE;
Class NS_DICTIONARY;
Class NS_MUTABLE_DICTIONARY;
Class NS_PORT;
Class NS_MACH_PORT;

__attribute__((constructor))
static void common_initializer()
{
  static int initialized = 0;
  if (!initialized) {

    NS_NUMBER = [NSNumber class];
    NS_ARRAY = [NSArray class];
    NS_MUTABLE_ARRAY = [NSMutableArray class];
    NS_STRING = [NSString class];
    NS_MUTABLE_STRING = [NSMutableString class];
    
    NS_SET = [NSSet class];
    NS_MUTABLE_SET = [NSMutableSet class];
    
    NS_DICTIONARY = [NSDictionary class];
    NS_MUTABLE_DICTIONARY= [NSMutableDictionary class];
    
    NS_URL = [NSURL class];
    NS_DATA = [NSData class];
    NS_MUTABLE_DATA = [NSMutableData class];
    NS_PROCESS_INFO = [NSProcessInfo class];
    NS_URL_SESSION = [NSURLSession class];
    NS_URL_CACHE = [NSURLCache class];
    
    NS_PORT = [NSPort class];
    NS_MACH_PORT = [NSMachPort class];
  }
}
NS_ASSUME_NONNULL_END