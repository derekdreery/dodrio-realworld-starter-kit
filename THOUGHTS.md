# Thoughts

 - ~~Generally~~ awesome!
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
 - Manually parsing routes is a pain, (EDIT) but actually not that much of a pain.
 - It would be handy to have a `Node::null` version of `Node` that the renderer ignores, so you
   could do things like
   ```rust
   let some_state = Some("a message");
   div(bump)
     .children([
       p(bump).children([text("Some text that is always there")]).finish(),
       match some_state {
         Some(msg) => p(bump).children([text(msg)]).finish(),
         None => Node::Null
       }
     ])
     .finish()
   ```
   rather than having to create and mutate a vector. It's a bit more immutable/functionaly.
 - `async/await` will be really cool - but I need to understand how they work. Say I do
   ```rust
   // update the dom
   state = new_state;
   // re-render immediately
   vdom.schedule_render();
   let yet_newer_state = await some_async_call_like_fetch(); // modulo syntax :'D
   state = yet_newer_state;
   vdom.schedule_render();
   ```
   does a render get scheduled immediately, before the async call returns? I assume so, because the
   underlying generator will yield to javascript, which will then invoke the renderer (is this how
   it works?). Also, how does the vdom live long enough. I don't understand `Pin` yet.

