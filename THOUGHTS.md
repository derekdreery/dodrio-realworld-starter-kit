# Thoughts

 - Generally awesome!
 - Easy to get lost when writing html - lots of boilerplate!
 - I really miss simple closures in javascript, stuff like
   ```js
   // This isn't actually how you use fetch
   const request = method => url => params => fetch({
     method,
     url,
     params
   )}
   const post = request("POST");
   const put = request("PUT");
   // ...
   ```
 - Could we just get rid of `finish` by doing the conversion in the library? I'm sure there's a
   reason this isn't done already.
 - Text formatting is fiddly - we need to save the formatted text somwhere that lives longer than
   `'bump`. EDIT I found the solution to this by chance from looking at the source - you can
   allocate using the bump allocator. Maybe some more docs ;).
 - Manually parsing routes is a pain.
