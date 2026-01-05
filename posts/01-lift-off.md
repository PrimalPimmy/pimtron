---
title: Lift Off! About this Site
date: 2025-12-30
slug: lift-off
summary: I talk about how I built this blog (P.S. it's not a JS framework)
---

## Preface

Welcome aboard! I made this ship with the dream of starting something new for the next year, but we all know how that goes xD. Hopefully, I'll be more consistent this year.

As you can see, announced below, this website is made with [**Leptos**](https://leptos.dev/); A Rust framework to build web applications. It's still in early release, but I found it quite impressive. I feel like WebAssembly has come a long way, and we don't have to depend on JavaScript as much as we did before.

As for the reason, well...there is no reason for me to choose this framework over the other. I believe everything has their own advantages, I chose this because I could. That's a good freedom to have. Not everything should have a reason. It should also be fun! I will however tell you how I made this website, since I really enjoyed making it.

## About Leptos

It was fairly simple, Leptos has amazing documentation here: [https://book.leptos.dev/](https://book.leptos.dev/), and I made a few dummy webapps with it in the past. Their discord server is also pretty neat to help out with your issues. I'd recommend checking them out.

Core concepts of Leptos are its Signal based reactivity. We know React Hooks rely on re-running the entire component function every time state changes to determine what needs to update. In contrast, Signals use fine-grained subscriptions to pinpoint the exact piece of data that changed and update only that specific part of the UI. You can learn more about the differences online, since I myself haven't dug deep into the advantages of both.

The purpose was to test the capabilities of WebAssembly for me, as even a JS framework could build this site. I've been working with WebGL/WebGPU as a side project to see how far we've come with GPU compute in browsers and I love how we've made steady progress! (WebGPU is now officially supported on many platforms, except Linux, sigh.)
 
## Working of Leptos

Leptos compiles your Rust code into a high-performance WebAssembly (WASM) binary that runs directly in the browser’s virtual machine. A small JavaScript "shim" loads this binary, which then uses direct browser APIs to manipulate the DOM without a slow Virtual DOM layer. This allows the app to execute complex logic at near-native speeds while maintaining the fine-grained reactivity of a modern web framework. Pretty neat right? Even though JavaScript dominates the web market, WebAssembly continues to grow.

Now I'm not knowledgeable enough to compare anything, so I'm going to keep that debate out. Maybe some day in a new blog I'll dig into it deeper.

#### About Rust in Leptos

So when coding out Leptos, they use a macro `view! {...}` - which allows us to write HTML styled markup as sort of a "Component" code, just like JSX/TSX! No build chains, everything is compiled by the rust compiler itself! Very neat feature.

In Leptos, components are just regular functions marked with `#[component]`.

Anything you want to be "live" or changeable is wrapped in a closure (an anonymous function). Because Leptos "listens" to which signals are being used inside those closures, it knows exactly which tiny piece of the UI needs to change. When a value updates, it doesn't re-render the whole page; it just tweaks that one specific spot in the DOM.

Since this website is made with CSR (Client Side Rendering), everything that happens is in the user's browser.

### Markdown

Ah yes. Good old markdown parsing. I have done this countless times while making my other blogs (which failed, canon event). Alright, let's do this one last time:

We whip up a custom binary [gen_posts.rs](https://github.com/PrimalPimmy/pimtron/blob/master/src/bin/gen_posts.rs) that runs before the main lift off of the site happens. The steps are as follows:

   1. We grab every .md file from the `posts/` folder.
   2. We use a crate called gray_matter to peel off the top part of the file; the "Frontmatter".
      That's where we keep the meta stuff like the title, date, and slug. The rest is the content.
   3. We feed that content into pulldown-cmark. It's a super fast markdown parser that turns our text into a stream of "events" (like "Start Paragraph", "Text", "End Paragraph").
   4. Syntax Highlighting. This is the fun part. As those events flow through, we watch for code blocks. When we see one, we hijack it! We use [syntect](https://github.com/trishume/syntect) to apply that lovely rose-pine theme to the code, turn it into colored HTML, and then slip it back into the stream.
   5. Finally, we turn those events back into raw HTML string and package everything up into a nice little JSON file in generated_posts/.

   > Now I could have have just hosted the markdown files in the browser and let the browser fetch that to make it simpler, but I don't know, it just didn't feel right.

   ### Fetching Site content

   I had two options, either embed everything into the WASM binary and inflate its size with each new content, or try to do some Lazy loading when a user enters my site, instead of downloading a big wasm binary at once. Hence, using json to generate posts was necessary to ensure I don't just embed everything to the wasm binary.

   Leptos has a feature of lazy loading and WASM splitting, but I could not get that to work, maybe later I'll try that again. So for now, I just lazy load the blog page, the about me page and the projects page, as soon as the user lands in the home page!

   ```rust
   fn App() -> impl IntoView {
    provide_meta_context();

    // fetch posts as soon as the app starts
    let posts_resource =
        LocalResource::new(|| async { crate::utils::content::fetch_all_posts().await });
    provide_context(posts_resource);

    // fetch About page
    let about_resource =
        LocalResource::new(|| async { crate::utils::content::fetch_about().await });
    provide_context(AboutResource(about_resource));

    // fetch Projects page
    let projects_resource =
        LocalResource::new(|| async { crate::utils::content::fetch_projects().await });
    provide_context(ProjectsResource(projects_resource));
   }
   ```

   More about Leptos' `LocalResource` feature [here](https://book.leptos.dev/async/10_resources.html)

   Now I know, what's the use of this overcomplication when the site is so small that synchronous loading runs fine too. And I'd say, there is no use. I just like to engineer it nicely :D 

   ### Styling

   Oh boy, where do I even begin. It's a mess. I've been doing Backend development all my career, I couldn't care less about styling before. But something about making this site `Retro Space` aesthetic is what made me do this.

   I don't know crap about CSS, not even a bit. I started off most of the CSS work with AI prototyping, 100s of broken effects which I had to fix manually, here I am. So I guess I did learn a lot of CSS along the way! Everything you see here is all my idea. From custom markdown styling to the stars and the ship diagram in the home page. Credits to [https://departuremono.com/](https://departuremono.com/) btw. Incredible site!

   But seriously, I hope I could be a good graphics person on my own. I'm learning graphics programming, so in a way, UI/UX is also a part of it. And the styling I've done here is a total mess. But it works, so there's that. `If it ain't broke, don't fix it` type situation.

   ### Deployment: The Launchpad

   Now, how does this ship actually get into orbit? I didn't want to just drag and drop files. I wanted a robust, reproducible launch sequence. Enter **Nix**.

   I use a `flake.nix` file to define my entire build environment. It uses [`crane`](https://crane.dev/) (a fantastic tool for containerizing Rust builds) to pull in the Rust toolchain, compile the WASM binary, and bundle everything with `trunk`. The beauty of this is that the build is "pure"; it doesn't care if I'm on my Linux desktop or a CI server; it produces the exact same binary, bit for bit.

   For the actual deployment, I have a GitHub Action that spins up a Nix environment, runs `nix build`, and then hands the resulting artifact over to Cloudflare Pages. It's fully automated. I push to `master`, and minutes later, the new version is live across the globe.

   It was a bit of a headache to set up initially (Nix always is, let's be honest), and the whole declarative config is neat, but absolutely not necessary for a small website like this lol. But I do see some improvements in build times, probably due to [magic-nix-cache](https://github.com/DeterminateSystems/magic-nix-cache).

   ### The end?

   Thank you for sticking through with me in this blog. This is probably my very first blog, and this website is my way of sharing my work out there. I hope I'll be able to write more about my future work!

   If you have any comments or questions, please feel free to [email me](mailto:prashant20.pm@gmail.com); I’d love to hear your feedback. Cheers!
