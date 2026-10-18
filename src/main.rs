use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;

#[component]
fn App() -> impl IntoView {
    let (is_dark, set_is_dark) = signal(false);
    let (mobile_menu_open, set_mobile_menu_open) = signal(false);

    // Signals for X, Y mouse/touch coordinates and scroll percentage
    let (mouse_x, set_mouse_x) = signal(0);
    let (mouse_y, set_mouse_y) = signal(0);
    let (scroll_pct, set_scroll_pct) = signal(0);

    // Mouse move listener
    let mouse_closure =
        web_sys::wasm_bindgen::closure::Closure::<dyn FnMut(_)>::new(move |e: web_sys::Event| {
            if let Some(mouse_event) = e.dyn_ref::<web_sys::MouseEvent>() {
                set_mouse_x.set(mouse_event.client_x());
                set_mouse_y.set(mouse_event.client_y());
            }
        });
    let _ = window()
        .add_event_listener_with_callback("mousemove", mouse_closure.as_ref().unchecked_ref());
    mouse_closure.forget();

    // Touch move listener
    let touch_closure =
        web_sys::wasm_bindgen::closure::Closure::<dyn FnMut(_)>::new(move |e: web_sys::Event| {
            if let Some(touch_event) = e.dyn_ref::<web_sys::TouchEvent>() {
                if let Some(touch) = touch_event.touches().get(0) {
                    set_mouse_x.set(touch.client_x());
                    set_mouse_y.set(touch.client_y());
                }
            }
        });
    let _ = window()
        .add_event_listener_with_callback("touchmove", touch_closure.as_ref().unchecked_ref());
    touch_closure.forget();

    // Scroll listener for 0% to 100% calculation
    let scroll_closure =
        web_sys::wasm_bindgen::closure::Closure::<dyn FnMut(_)>::new(move |_e: web_sys::Event| {
            if let Some(doc) = document().document_element() {
                let scroll_top = doc.scroll_top() as f64;
                let scroll_height = (doc.scroll_height() - doc.client_height()) as f64;
                if scroll_height > 0.0 {
                    let pct = ((scroll_top / scroll_height) * 100.0).round() as i32;
                    set_scroll_pct.set(pct.clamp(0, 100));
                } else {
                    set_scroll_pct.set(0);
                }
            }
        });
    let _ = window()
        .add_event_listener_with_callback("scroll", scroll_closure.as_ref().unchecked_ref());
    scroll_closure.forget();

    let toggle_theme = move |_| {
        let next_state = !is_dark.get();
        set_is_dark.set(next_state);
        let html_element = document().document_element().unwrap();
        if next_state {
            let _ = html_element.class_list().add_1("dark");
        } else {
            let _ = html_element.class_list().remove_1("dark");
        }
    };

    view! {
        // Floating Upper-Left Mouse/Touch Coordinates Indicator with Green Glow
        <div class="fixed top-24 left-4 z-10 px-3 py-1.5 rounded-xl bg-white/80 dark:bg-slate-900/80 backdrop-blur-md shadow-lg shadow-emerald-500/10 border border-emerald-500/30 text-xs font-mono text-slate-700 dark:text-slate-300 pointer-events-none">
            <div>"X: " {mouse_x}</div>
            <div>"Y: " {mouse_y}</div>
        </div>

        // Floating Lower-Right Scroll Percentage Circle Indicator with Green Glow
        <div class="fixed bottom-6 right-6 z-50 w-16 h-16 rounded-full bg-white/90 dark:bg-slate-900/90 backdrop-blur-md shadow-2xl shadow-emerald-500/20 border border-emerald-500/40 flex items-center justify-center pointer-events-none">
            <span class="text-xs font-bold text-emerald-600 dark:text-emerald-400">
                {move || format!("{}%", scroll_pct.get())}
            </span>
        </div>

        <header class="sticky top-0 z-40 backdrop-blur-md bg-white/80 dark:bg-slate-900/80 border-b border-emerald-500/20 dark:border-emerald-500/20 transition-colors">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 h-20 flex items-center justify-between">
                <a href="#home" class="flex items-center gap-3 group">
                    <div class="w-10 h-10 rounded-full bg-gradient-to-tr from-emerald-500 to-teal-600 text-white flex items-center justify-center font-serif font-bold text-lg shadow-md shadow-emerald-500/30">
                        "LA"
                    </div>
                    <span class="font-serif font-bold text-xl sm:text-2xl tracking-tight text-slate-900 dark:text-white">
                        Leemark <span class="text-emerald-600 dark:text-emerald-400 font-sans font-medium text-base sm:text-lg block sm:inline sm:ml-1">"Arojo"</span>
                    </span>
                </a>

                <nav class="hidden md:flex items-center gap-2">
                    <a href="#about" class="px-3 py-2 text-sm font-medium rounded-full text-slate-700 dark:text-slate-200 hover:text-emerald-600 dark:hover:text-emerald-400 transition-all">"About"</a>
                    <a href="#skills" class="px-3 py-2 text-sm font-medium rounded-full text-slate-700 dark:text-slate-200 hover:text-emerald-600 dark:hover:text-emerald-400 transition-all">"Tech Stack"</a>
                    <a href="#projects" class="px-3 py-2 text-sm font-medium rounded-full text-slate-700 dark:text-slate-200 hover:text-emerald-600 dark:hover:text-emerald-400 transition-all">"Projects"</a>
                    <a href="#experience" class="px-3 py-2 text-sm font-medium rounded-full text-slate-700 dark:text-slate-200 hover:text-emerald-600 dark:hover:text-emerald-400 transition-all">"Experience"</a>
                    <a href="#contact" class="px-4 py-2 text-sm font-semibold rounded-full bg-gradient-to-r from-emerald-500 to-teal-600 text-white shadow-md shadow-emerald-500/25 hover:shadow-lg hover:shadow-emerald-500/40 transition-all ml-2">"Contact Me"</a>

                    <span on:click=toggle_theme class="ml-3 cursor-pointer text-slate-600 dark:text-slate-300 hover:text-emerald-500 transition-colors">
                        {move || if is_dark.get() {
                            view! { <i class="fa-solid fa-sun text-amber-400"></i> }.into_view()
                        } else {
                            view! { <i class="fa-solid fa-moon"></i> }.into_view()
                        }}
                    </span>
                </nav>

                <div class="flex items-center gap-2 md:hidden">
                    <button on:click=toggle_theme class="p-2 rounded-full bg-slate-100 dark:bg-slate-800 text-slate-600 dark:text-slate-300">
                        {move || if is_dark.get() {
                            view! { <i class="fa-solid fa-sun text-amber-400"></i> }.into_view()
                        } else {
                            view! { <i class="fa-solid fa-moon"></i> }.into_view()
                        }}
                    </button>
                    <button on:click=move |_| set_mobile_menu_open.update(|v| *v = !*v) class="p-2 rounded-lg text-slate-700 dark:text-slate-200">
                        <i class="fa-solid fa-bars text-xl"></i>
                    </button>
                </div>
            </div>

            <div class={move || format!("md:hidden border-b border-emerald-500/20 bg-white dark:bg-slate-900 px-4 pt-2 pb-6 space-y-2 {}", if mobile_menu_open.get() { "" } else { "hidden" })}>
                <a href="#about" on:click=move |_| set_mobile_menu_open.set(false) class="block px-4 py-2 rounded-xl text-slate-700 dark:text-slate-200">"About"</a>
                <a href="#skills" on:click=move |_| set_mobile_menu_open.set(false) class="block px-4 py-2 rounded-xl text-slate-700 dark:text-slate-200">"Tech Stack"</a>
                <a href="#projects" on:click=move |_| set_mobile_menu_open.set(false) class="block px-4 py-2 rounded-xl text-slate-700 dark:text-slate-200">"Projects"</a>
                <a href="#experience" on:click=move |_| set_mobile_menu_open.set(false) class="block px-4 py-2 rounded-xl text-slate-700 dark:text-slate-200">"Experience"</a>
                <a href="#contact" on:click=move |_| set_mobile_menu_open.set(false) class="block px-4 py-2 rounded-xl bg-emerald-600 text-white text-center shadow-md shadow-emerald-500/30">"Contact Me"</a>
            </div>
        </header>

        <main class="py-12 px-4 sm:px-6 lg:px-8 max-w-5xl mx-auto space-y-32 relative z-10">
            // HERO SECTION
            // HERO SECTION
                        <section id="home" class="min-h-[65vh] flex flex-col justify-center space-y-6 pt-2">
                            <div class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-emerald-50 dark:bg-emerald-950/40 border border-emerald-500/30 text-emerald-600 dark:text-emerald-300 text-sm font-semibold shadow-sm shadow-emerald-500/15 w-fit">
                                <span class="w-2 h-2 rounded-full bg-emerald-500 animate-pulse"></span> "Available for work (Remote / Freelance / Full-time)"
                            </div>

                            <div class="flex flex-col md:flex-row items-center gap-8 pt-2">
                            <div class="relative w-36 h-36 sm:w-44 sm:h-44 shrink-0 rounded-full p-1 bg-gradient-to-tr from-emerald-500 to-teal-600 shadow-xl shadow-emerald-500/20 group">
                                <div class="w-full h-full rounded-full overflow-hidden bg-white dark:bg-slate-900 border-2 border-white dark:border-slate-900">
                                  <img src="/leemark-portfolio/rustdev.png" alt="Leemark E. Arojo" class="w-full h-full object-cover contrast-110 group-hover:scale-105 transition-transform duration-300"/> alt="Leemark E. Arojo" class="w-full h-full object-cover contrast-110 group-hover:scale-105 transition-transform duration-300"/>
                                </div>
                            </div>
                                <div class="space-y-4 text-center md:text-left">
                                    <h1 class="text-4xl sm:text-6xl font-extrabold text-slate-900 dark:text-white tracking-tight">
                                        "Hi, I'm " <span class="text-emerald-600 dark:text-emerald-400 drop-shadow-[0_0_15px_rgba(16,185,129,0.3)]">"Leemark"</span>
                                    </h1>
                                    <div>
                                        <span class="inline-block px-3 py-1 rounded-md bg-emerald-500/10 border border-emerald-500/20 font-mono text-sm sm:text-base font-semibold text-emerald-600 dark:text-emerald-400">
                                            "fn main() -> Role { \"Rust Full-Stack Developer\" }"
                                        </span>
                                    </div>
                                    <p class="text-base text-slate-600 dark:text-slate-300 leading-relaxed max-w-2xl">
                                        "I build high-performance web applications and backend systems in Rust using Axum, Leptos, and PostgreSQL, focusing on performance, maintainability, and clean architecture."
                                    </p>

                                    // Action Buttons & Social Links Container
                                    <div class="flex flex-wrap items-center justify-center md:justify-start gap-4 pt-2">
                                        <a href="#projects" class="px-6 py-3 rounded-full bg-gradient-to-r from-emerald-600 to-teal-600 text-white font-bold shadow-lg shadow-emerald-500/30 hover:shadow-emerald-500/50 hover:scale-[1.02] transition-all flex items-center gap-2">
                                            <i class="fa-solid fa-code"></i> "View Projects"
                                        </a>
                                        <a href="#contact" class="px-6 py-3 rounded-full border border-emerald-500/40 hover:border-emerald-500 text-slate-800 dark:text-slate-200 font-semibold hover:bg-emerald-500/10 transition-all flex items-center gap-2">
                                            <i class="fa-solid fa-paper-plane"></i> "Contact Me"
                                        </a>

                                        // Social Media Links
                                        <div class="flex items-center gap-3 pl-0 md:pl-2">
                                            <a href="https://github.com/Arkleem" target="_blank" rel="noopener noreferrer" class="p-3 rounded-full bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 hover:text-emerald-500 dark:hover:text-emerald-400 border border-slate-200 dark:border-slate-700 hover:border-emerald-500/50 transition-all" title="GitHub">
                                                <i class="fa-brands fa-github text-lg"></i>
                                            </a>
                                            <a href="https://www.linkedin.com/in/leemarkarojo/" target="_blank" rel="noopener noreferrer" class="p-3 rounded-full bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 hover:text-emerald-500 dark:hover:text-emerald-400 border border-slate-200 dark:border-slate-700 hover:border-emerald-500/50 transition-all" title="LinkedIn">
                                                <i class="fa-brands fa-linkedin text-lg"></i>
                                            </a>
                                            <a href="https://www.facebook.com/krameel28/" target="_blank" rel="noopener noreferrer" class="p-3 rounded-full bg-slate-100 dark:bg-slate-800 text-slate-700 dark:text-slate-300 hover:text-emerald-500 dark:hover:text-emerald-400 border border-slate-200 dark:border-slate-700 hover:border-emerald-500/50 transition-all" title="Facebook">
                                                <i class="fa-brands fa-facebook text-lg"></i>
                                            </a>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </section>
                        // ABOUT ME SECTION (Consistent Terminal Theme)
                        // ABOUT ME SECTION (Adaptive Light/Dark Terminal Theme)
                                    <section id="about" class="min-h-[75vh] flex flex-col justify-center space-y-6 pt-10">
                                        <h2 class="text-2xl font-bold text-slate-900 dark:text-white border-b border-emerald-500/30 pb-3 flex items-center gap-2">
                                            <i class="fa-solid fa-terminal text-emerald-600 dark:text-emerald-400"></i> "About Me"
                                        </h2>

                                        // Terminal Container with Adaptive Background
                                        <div class="rounded-2xl overflow-hidden bg-emerald-50 dark:bg-emerald-950/40 backdrop-blur-md shadow-xl shadow-emerald-500/10 border border-emerald-500/30 font-mono text-xs sm:text-sm">
                                            // Terminal Header Bar
                                            <div class="bg-emerald-100/60 dark:bg-black/40 px-4 py-3 border-b border-emerald-500/20 flex items-center justify-between">
                                                <div class="flex items-center gap-2">
                                                    <span class="w-3 h-3 rounded-full bg-red-500/90 inline-block"></span>
                                                    <span class="w-3 h-3 rounded-full bg-amber-500/90 inline-block"></span>
                                                    <span class="w-3 h-3 rounded-full bg-emerald-500/90 inline-block"></span>
                                                </div>
                                                <div class="text-emerald-700 dark:text-emerald-400/90 text-xs font-semibold tracking-wide">
                                                    "leemark@don: ~"
                                                </div>
                                                <div class="text-emerald-600/60 dark:text-emerald-500/60 text-xs hidden sm:block">
                                                    "bash"
                                                </div>
                                            </div>

                                            // Terminal Content Body
                                            <div class="p-6 sm:p-8 space-y-6 text-slate-800 dark:text-slate-100 leading-relaxed">
                                                // Command Line 1
                                                <div class="space-y-2">
                                                    <div class="flex items-center gap-1.5 font-medium">
                                                        <span class="text-slate-700 dark:text-slate-200">"leemark@don:~$"</span>
                                                        <span class="text-emerald-600 dark:text-emerald-400 font-semibold ml-1">"cat about_me.txt"</span>
                                                    </div>
                                                    <p class="text-slate-700 dark:text-slate-200/90 pl-4 border-l-2 border-emerald-500/60">
                                                        "Computer Science graduate specializing in "
                                                        <span class="text-emerald-600 dark:text-emerald-400 font-semibold">"Rust development"</span>
                                                        " and reliable software systems. Skilled in building backend services, REST APIs, database-driven applications, and full-stack web architectures using "
                                                        <span class="text-emerald-600 dark:text-emerald-300 font-semibold">"Rust"</span>", "
                                                        <span class="text-emerald-600 dark:text-emerald-300 font-semibold">"Axum"</span>", "
                                                        <span class="text-emerald-600 dark:text-emerald-300 font-semibold">"Tokio"</span>", "
                                                        <span class="text-emerald-600 dark:text-emerald-300 font-semibold">"Leptos"</span>", and "
                                                        <span class="text-emerald-600 dark:text-emerald-300 font-semibold">"PostgreSQL"</span>
                                                        ". Experienced in designing clean, maintainable software architectures with a focus on performance, scalability, and asynchronous programming."
                                                    </p>
                                                </div>

                                                // Command Line 2
                                                <div class="space-y-2 pt-2">
                                                    <div class="flex items-center gap-1.5 font-medium">
                                                        <span class="text-slate-700 dark:text-slate-200">"leemark@don:~$"</span>
                                                        <span class="text-emerald-600 dark:text-emerald-400 font-semibold ml-1">"cat workflow.log"</span>
                                                    </div>
                                                    <p class="text-slate-700 dark:text-slate-200/90 pl-4 border-l-2 border-teal-500/60">
                                                        "Proficient with "
                                                        <span class="text-emerald-600 dark:text-emerald-400 font-semibold">"Linux"</span>
                                                        " and modern development workflows. Passionate about backend engineering and eager to contribute to building high-quality software solutions while continuously learning and growing as a software engineer."
                                                    </p>
                                                </div>

                                                // Active Line with Solid Cursor Block
                                                <div class="flex items-center gap-1.5 pt-2">
                                                    <span class="text-slate-700 dark:text-slate-200 font-medium">"leemark@don:~$"</span>
                                                    <span class="w-2.5 h-4 bg-slate-800 dark:bg-slate-100 inline-block animate-pulse ml-0.5"></span>
                                                </div>
                                            </div>
                                        </div>
                                    </section>

                                    // TECH STACK SECTION (Matched Terminal Theme)
                                    // TECH STACK SECTION (Adaptive Light/Dark Terminal Theme)
                                                <section id="skills" class="min-h-[80vh] flex flex-col justify-center space-y-6 pt-10">
                                                    <h2 class="text-2xl font-bold text-slate-900 dark:text-white border-b border-emerald-500/30 pb-3 flex items-center gap-2">
                                                        <i class="fa-solid fa-code text-emerald-600 dark:text-emerald-400"></i> "Tech Stack"
                                                    </h2>

                                                    // Terminal Window Container with Adaptive Background
                                                    <div class="rounded-2xl overflow-hidden bg-emerald-50 dark:bg-emerald-950/40 backdrop-blur-md shadow-xl shadow-emerald-500/10 border border-emerald-500/30 font-mono text-xs sm:text-sm">
                                                        // Terminal Header Bar
                                                        <div class="bg-emerald-100/60 dark:bg-black/40 px-4 py-3 border-b border-emerald-500/20 flex items-center justify-between">
                                                            <div class="flex items-center gap-2">
                                                                <span class="w-3 h-3 rounded-full bg-red-500/90 inline-block"></span>
                                                                <span class="w-3 h-3 rounded-full bg-amber-500/90 inline-block"></span>
                                                                <span class="w-3 h-3 rounded-full bg-emerald-500/90 inline-block"></span>
                                                            </div>
                                                            <div class="text-emerald-700 dark:text-emerald-400/90 text-xs font-semibold tracking-wide">
                                                                "leemark@don: ~/tech-stack"
                                                            </div>
                                                            <div class="text-emerald-600/60 dark:text-emerald-500/60 text-xs hidden sm:block">
                                                                "bash"
                                                            </div>
                                                        </div>

                                                        // Terminal Content Body
                                                        <div class="p-6 sm:p-8 space-y-6 text-slate-800 dark:text-slate-100">
                                                            // Category 1: Core Technologies
                                                            <div class="space-y-3">
                                                                <div class="flex items-center gap-1.5 font-medium">
                                                                    <span class="text-slate-700 dark:text-slate-200">"leemark@don:~/tech-stack$"</span>
                                                                    <span class="text-emerald-600 dark:text-emerald-400 font-semibold ml-1">"ls core-technologies/"</span>
                                                                </div>
                                                                <div class="flex flex-wrap gap-2.5 pl-4 border-l-2 border-emerald-500/60 pt-1">
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-emerald-100 dark:bg-emerald-900/60 text-emerald-800 dark:text-emerald-300 border border-emerald-500/40">"Rust"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-teal-100 dark:bg-teal-900/60 text-teal-800 dark:text-teal-300 border border-teal-500/40">"Axum"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-green-100 dark:bg-green-900/60 text-green-800 dark:text-green-300 border border-green-500/40">"Leptos"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-emerald-100 dark:bg-emerald-900/60 text-emerald-800 dark:text-emerald-300 border border-emerald-500/40">"Tokio"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-cyan-100 dark:bg-cyan-900/60 text-cyan-800 dark:text-cyan-300 border border-cyan-500/40">"SQLx"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-blue-100 dark:bg-blue-900/60 text-blue-800 dark:text-blue-300 border border-blue-500/40">"PostgreSQL"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-emerald-100 dark:bg-emerald-900/60 text-emerald-800 dark:text-emerald-300 border border-emerald-500/40">"HTML/CSS"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-amber-100 dark:bg-amber-900/60 text-amber-800 dark:text-amber-300 border border-amber-500/40">"SurrealDB"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-orange-100 dark:bg-orange-900/60 text-orange-800 dark:text-orange-300 border border-orange-500/40">"SurrealQL"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-lime-100 dark:bg-lime-900/60 text-lime-800 dark:text-lime-300 border border-lime-500/40">"SurrealKV"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-slate-200 dark:bg-slate-800/80 text-slate-800 dark:text-slate-300 border border-slate-300 dark:border-slate-700">"Linux (Fedora)"</span>
                                                                </div>
                                                            </div>

                                                            // Category 2: Tools & Platforms
                                                            <div class="space-y-3 pt-3">
                                                                <div class="flex items-center gap-1.5 font-medium">
                                                                    <span class="text-slate-700 dark:text-slate-200">"leemark@don:~/tech-stack$"</span>
                                                                    <span class="text-emerald-600 dark:text-emerald-400 font-semibold ml-1">"ls tools-platforms/"</span>
                                                                </div>
                                                                <div class="flex flex-wrap gap-2.5 pl-4 border-l-2 border-teal-500/60 pt-1">
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-slate-200 dark:bg-slate-800/80 text-slate-800 dark:text-slate-300 border border-slate-300 dark:border-slate-700">"Git"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-slate-200 dark:bg-slate-800/80 text-slate-800 dark:text-slate-300 border border-slate-300 dark:border-slate-700">"GitHub"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-slate-200 dark:bg-slate-800/80 text-slate-800 dark:text-slate-300 border border-slate-300 dark:border-slate-700">"Cargo"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-slate-200 dark:bg-slate-800/80 text-slate-800 dark:text-slate-300 border border-slate-300 dark:border-slate-700">"Linux VPS"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-slate-200 dark:bg-slate-800/80 text-slate-800 dark:text-slate-300 border border-slate-300 dark:border-slate-700">"Zed IDE"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-slate-200 dark:bg-slate-800/80 text-slate-800 dark:text-slate-300 border border-slate-300 dark:border-slate-700">"Railway"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-slate-200 dark:bg-slate-800/80 text-slate-800 dark:text-slate-300 border border-slate-300 dark:border-slate-700">"Podman"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-slate-200 dark:bg-slate-800/80 text-slate-800 dark:text-slate-300 border border-slate-300 dark:border-slate-700">"DigitalOcean"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-slate-200 dark:bg-slate-800/80 text-slate-800 dark:text-slate-300 border border-slate-300 dark:border-slate-700">"Render"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-slate-200 dark:bg-slate-800/80 text-slate-800 dark:text-slate-300 border border-slate-300 dark:border-slate-700">"Shuttle"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-slate-200 dark:bg-slate-800/80 text-slate-800 dark:text-slate-300 border border-slate-300 dark:border-slate-700">"Cloudflare"</span>
                                                                    <span class="px-3 py-1.5 rounded-xl text-xs font-semibold bg-slate-200 dark:bg-slate-800/80 text-slate-800 dark:text-slate-300 border border-slate-300 dark:border-slate-700">"AI Code Agents"</span>
                                                                </div>
                                                            </div>

                                                            // Active Prompt Line with Cursor
                                                            <div class="flex items-center gap-1.5 pt-3">
                                                                <span class="text-slate-700 dark:text-slate-200 font-medium">"leemark@don:~/tech-stack$"</span>
                                                                <span class="w-2.5 h-4 bg-slate-800 dark:bg-slate-100 inline-block animate-pulse ml-0.5"></span>
                                                            </div>
                                                        </div>
                                                    </div>
                                                </section>
                                                // PROJECTS SECTION (Terminal Command Output View)
                                                // PROJECTS SECTION (Adaptive Light/Dark Terminal Theme)
                                                            <section id="projects" class="min-h-[85vh] flex flex-col justify-center space-y-6 pt-10">
                                                                <h2 class="text-2xl font-bold text-slate-900 dark:text-white border-b border-emerald-500/30 pb-3 flex items-center gap-2">
                                                                    <i class="fa-solid fa-diagram-project text-emerald-600 dark:text-emerald-400"></i> "My Projects"
                                                                </h2>

                                                                // Terminal Window Container with Adaptive Background
                                                                <div class="rounded-2xl overflow-hidden bg-emerald-50 dark:bg-emerald-950/40 backdrop-blur-md shadow-xl shadow-emerald-500/10 border border-emerald-500/30 font-mono text-xs sm:text-sm">
                                                                    // Terminal Header Bar
                                                                    <div class="bg-emerald-100/60 dark:bg-black/40 px-4 py-3 border-b border-emerald-500/20 flex items-center justify-between">
                                                                        <div class="flex items-center gap-2">
                                                                            <span class="w-3 h-3 rounded-full bg-red-500/90 inline-block"></span>
                                                                            <span class="w-3 h-3 rounded-full bg-amber-500/90 inline-block"></span>
                                                                            <span class="w-3 h-3 rounded-full bg-emerald-500/90 inline-block"></span>
                                                                        </div>
                                                                        <div class="text-emerald-700 dark:text-emerald-400/90 text-xs font-semibold tracking-wide">
                                                                            "leemark@don: ~/projects"
                                                                        </div>
                                                                        <div class="text-emerald-600/60 dark:text-emerald-500/60 text-xs hidden sm:block">
                                                                            "cargo run --bin showcase"
                                                                        </div>
                                                                    </div>

                                                                    // Terminal Content Body
                                                                    <div class="p-6 sm:p-8 space-y-6 text-slate-800 dark:text-slate-100">
                                                                        <div class="flex items-center gap-1.5 font-medium">
                                                                            <span class="text-slate-700 dark:text-slate-200">"leemark@don:~/projects$"</span>
                                                                            <span class="text-emerald-600 dark:text-emerald-400 font-semibold ml-1">"cargo run --example projects"</span>
                                                                        </div>

                                                                        // Grid of Project Terminal Cards
                                                                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 pt-2">
                                                                            // Project 1
                                                                            <div class="p-5 rounded-xl bg-white/70 dark:bg-black/40 border border-emerald-500/30 space-y-4 flex flex-col justify-between hover:border-emerald-500/60 transition-all shadow-sm">
                                                                                <div class="space-y-3">
                                                                                    <div class="overflow-hidden rounded-lg border border-emerald-500/30 cursor-pointer group">
                                                                                        <img src="https://images.unsplash.com/photo-1555066931-4365d14bab8c?auto=format&fit=crop&w=800&q=80" alt="Portfolio Preview" class="w-full h-40 object-cover group-hover:scale-105 transition-transform duration-300"/>
                                                                                    </div>
                                                                                    <span class="text-xs font-bold text-emerald-600 dark:text-emerald-400">"Rust | Axum | Leptos"</span>
                                                                                    <h3 class="text-lg font-bold text-slate-900 dark:text-slate-100">"Portfolio Website"</h3>
                                                                                    <p class="text-xs text-slate-600 dark:text-slate-300/90 leading-relaxed font-sans">
                                                                                        "Modern portfolio built with Rust, Axum, and Leptos featuring full-stack interactivity and reactive client-side states."
                                                                                    </p>
                                                                                </div>
                                                                                <button class="text-xs font-semibold text-emerald-600 dark:text-emerald-400 hover:underline flex items-center gap-1 self-start pt-2">
                                                                                    <i class="fa-solid fa-expand"></i> "Click image to expand"
                                                                                </button>
                                                                            </div>

                                                                            // Project 2
                                                                            <div class="p-5 rounded-xl bg-white/70 dark:bg-black/40 border border-emerald-500/30 space-y-4 flex flex-col justify-between hover:border-emerald-500/60 transition-all shadow-sm">
                                                                                <div class="space-y-3">
                                                                                    <div class="overflow-hidden rounded-lg border border-emerald-500/30 cursor-pointer group">
                                                                                        <img src="https://images.unsplash.com/photo-1507238691740-187a5b1d37b8?auto=format&fit=crop&w=800&q=80" alt="Task App Preview" class="w-full h-40 object-cover group-hover:scale-105 transition-transform duration-300"/>
                                                                                    </div>
                                                                                    <span class="text-xs font-bold text-teal-600 dark:text-teal-300">"Rust | Axum | SurrealDB"</span>
                                                                                    <h3 class="text-lg font-bold text-slate-900 dark:text-slate-100">"Task Management App"</h3>
                                                                                    <p class="text-xs text-slate-600 dark:text-slate-300/90 leading-relaxed font-sans">
                                                                                        "Full-featured task management application with team collaboration features and a modern server-backed architecture built with Axum and SurrealDB."
                                                                                    </p>
                                                                                </div>
                                                                                <button class="text-xs font-semibold text-teal-600 dark:text-teal-300 hover:underline flex items-center gap-1 self-start pt-2">
                                                                                    <i class="fa-solid fa-expand"></i> "Click image to expand"
                                                                                </button>
                                                                            </div>

                                                                            // Project 3 (Full Width)
                                                                            <div class="p-5 rounded-xl bg-white/70 dark:bg-black/40 border border-emerald-500/30 space-y-4 flex flex-col justify-between md:col-span-2 hover:border-emerald-500/60 transition-all shadow-sm">
                                                                                <div class="space-y-3">
                                                                                    <div class="overflow-hidden rounded-lg border border-emerald-500/30 cursor-pointer group">
                                                                                        <img src="https://images.unsplash.com/photo-1551288049-bebda4e38f71?auto=format&fit=crop&w=1200&q=80" alt="PCRT System Preview" class="w-full h-48 object-cover group-hover:scale-105 transition-transform duration-300"/>
                                                                                    </div>
                                                                                    <span class="text-xs font-bold text-green-600 dark:text-green-400">"Rust | Axum | SurrealDB"</span>
                                                                                    <h3 class="text-lg font-bold text-slate-900 dark:text-slate-100">"PCRT Record Management System"</h3>
                                                                                    <p class="text-xs text-slate-600 dark:text-slate-300/90 leading-relaxed font-sans">
                                                                                        "Collaborated with security analysts and network teams to design full-stack data tools. Developed high-performance technical dashboards leveraging Rust to track cybercrime response workflows."
                                                                                    </p>
                                                                                </div>
                                                                                <button class="text-xs font-semibold text-green-600 dark:text-green-400 hover:underline flex items-center gap-1 self-start pt-2">
                                                                                    <i class="fa-solid fa-expand"></i> "Click image to expand"
                                                                                </button>
                                                                            </div>
                                                                        </div>

                                                                        // Active Terminal Line with Cursor
                                                                        <div class="flex items-center gap-1.5 pt-3">
                                                                            <span class="text-slate-700 dark:text-slate-200 font-medium">"leemark@don:~/projects$"</span>
                                                                            <span class="w-2.5 h-4 bg-slate-800 dark:bg-slate-100 inline-block animate-pulse ml-0.5"></span>
                                                                        </div>
                                                                    </div>
                                                                </div>
                                                            </section>
            // EXPERIENCE & EDUCATION SECTION
            // EXPERIENCE & EDUCATION SECTION (Adaptive Light/Dark Terminal Theme)
                        <section id="experience" class="min-h-[85vh] flex flex-col justify-center space-y-6 pt-10">
                            <h2 class="text-2xl font-bold text-slate-900 dark:text-white border-b border-emerald-500/30 pb-3 flex items-center gap-2">
                                <i class="fa-solid fa-briefcase text-emerald-600 dark:text-emerald-400"></i> "Experience & Education"
                            </h2>

                            // Terminal Window Container with Adaptive Background
                            <div class="rounded-2xl overflow-hidden bg-emerald-50 dark:bg-emerald-950/40 backdrop-blur-md shadow-xl shadow-emerald-500/10 border border-emerald-500/30 font-mono text-xs sm:text-sm">
                                // Terminal Header Bar
                                <div class="bg-emerald-100/60 dark:bg-black/40 px-4 py-3 border-b border-emerald-500/20 flex items-center justify-between">
                                    <div class="flex items-center gap-2">
                                        <span class="w-3 h-3 rounded-full bg-red-500/90 inline-block"></span>
                                        <span class="w-3 h-3 rounded-full bg-amber-500/90 inline-block"></span>
                                        <span class="w-3 h-3 rounded-full bg-emerald-500/90 inline-block"></span>
                                    </div>
                                    <div class="text-emerald-700 dark:text-emerald-400/90 text-xs font-semibold tracking-wide">
                                        "leemark@don: ~/career"
                                    </div>
                                    <div class="text-emerald-600/60 dark:text-emerald-500/60 text-xs hidden sm:block">
                                        "bash"
                                    </div>
                                </div>

                                // Terminal Content Body
                                <div class="p-6 sm:p-8 space-y-6 text-slate-800 dark:text-slate-100">
                                    <div class="space-y-6">
                                        // Work Experience Sub-block
                                        <div class="space-y-3">
                                            <div class="flex items-center gap-1.5 font-medium">
                                                <span class="text-slate-700 dark:text-slate-200">"leemark@don:~/career$"</span>
                                                <span class="text-emerald-600 dark:text-emerald-400 font-semibold ml-1">"cat work_history.log"</span>
                                            </div>
                                            <div class="p-5 rounded-xl bg-white/70 dark:bg-black/40 border border-emerald-500/30 space-y-3 pl-4 border-l-2 border-l-emerald-500 shadow-sm">
                                                <span class="text-xs font-bold text-emerald-600 dark:text-emerald-400">"December 2025 - February 2026"</span>
                                                <h4 class="text-lg font-bold text-slate-900 dark:text-slate-100">"Software Developer Intern"</h4>
                                                <p class="text-xs text-emerald-700 dark:text-emerald-300/80">"Zamboanga Sibugay Provincial Cyber Response Team (Taway, Ipil)"</p>
                                                <ul class="list-disc list-inside text-xs text-slate-600 dark:text-slate-300/90 space-y-1 font-sans">
                                                    <li>"Built a real-time operational dashboard using Rust, Axum, and SurrealDB"</li>
                                                    <li>"Optimized database queries and record retrieval workflows, improving dashboard response times for internal operations"</li>
                                                    <li>"Implemented role-based access control supporting 3 user levels: Admin, Investigator, and Viewer"</li>
                                                </ul>
                                            </div>
                                        </div>

                                        // Education Sub-block
                                        <div class="space-y-3 pt-2">
                                            <div class="flex items-center gap-1.5 font-medium">
                                                <span class="text-slate-700 dark:text-slate-200">"leemark@don:~/career$"</span>
                                                <span class="text-emerald-600 dark:text-emerald-400 font-semibold ml-1">"cat education.txt"</span>
                                            </div>
                                            <div class="p-5 rounded-xl bg-white/70 dark:bg-black/40 border border-emerald-500/30 space-y-3 pl-4 border-l-2 border-l-teal-500 shadow-sm">
                                                <span class="text-xs font-bold text-teal-600 dark:text-teal-300">"June 2023 - April 2026 | With Honors"</span>
                                                <h4 class="text-lg font-bold text-slate-900 dark:text-slate-100">"Dr. Aurelio Mendoza Memorial Colleges"</h4>
                                                <p class="text-xs text-emerald-700 dark:text-emerald-300/80">"Bachelor of Science in Computer Science, Major in Software Engineering"</p>
                                                <p class="text-xs text-slate-500 dark:text-slate-400">"Ipil, Zamboanga Sibugay"</p>
                                                <div class="pt-2 text-xs text-slate-600 dark:text-slate-300/90 font-sans">
                                                    <strong class="block mb-1 text-emerald-600 dark:text-emerald-400 font-mono">"Relevant Coursework:"</strong>
                                                    "Data Structures & Algorithms | Software Engineering | Operating Systems | Database Systems | Web & Mobile Development"
                                                </div>
                                            </div>
                                        </div>
                                    </div>

                                    // Terminal Line Cursor
                                    <div class="flex items-center gap-1.5 pt-3">
                                        <span class="text-slate-700 dark:text-slate-200 font-medium">"leemark@don:~/career$"</span>
                                        <span class="w-2.5 h-4 bg-slate-800 dark:bg-slate-100 inline-block animate-pulse ml-0.5"></span>
                                    </div>
                                </div>
                            </div>
                        </section>
                        // CONTACT SECTION (Adaptive Light/Dark Terminal Theme)
                                    <section id="contact" class="min-h-[60vh] flex flex-col justify-center space-y-6 pt-10">
                                        <h2 class="text-2xl font-bold text-slate-900 dark:text-white border-b border-emerald-500/30 pb-3 flex items-center gap-2">
                                            <i class="fa-solid fa-envelope text-emerald-600 dark:text-emerald-400"></i> "Get In Touch"
                                        </h2>

                                        // Terminal Window Container with Adaptive Background
                                        <div class="rounded-2xl overflow-hidden bg-emerald-50 dark:bg-emerald-950/40 backdrop-blur-md shadow-xl shadow-emerald-500/10 border border-emerald-500/30 font-mono text-xs sm:text-sm">
                                            // Terminal Header Bar
                                            <div class="bg-emerald-100/60 dark:bg-black/40 px-4 py-3 border-b border-emerald-500/20 flex items-center justify-between">
                                                <div class="flex items-center gap-2">
                                                    <span class="w-3 h-3 rounded-full bg-red-500/90 inline-block"></span>
                                                    <span class="w-3 h-3 rounded-full bg-amber-500/90 inline-block"></span>
                                                    <span class="w-3 h-3 rounded-full bg-emerald-500/90 inline-block"></span>
                                                </div>
                                                <div class="text-emerald-700 dark:text-emerald-400/90 text-xs font-semibold tracking-wide">
                                                    "leemark@don: ~/contact"
                                                </div>
                                                <div class="text-emerald-600/60 dark:text-emerald-500/60 text-xs hidden sm:block">
                                                    "curl"
                                                </div>
                                            </div>

                                            // Terminal Content Body
                                            <div class="p-6 sm:p-8 space-y-6 text-slate-800 dark:text-slate-100 text-center sm:text-left">
                                                <div class="flex items-center gap-1.5 font-medium">
                                                    <span class="text-slate-700 dark:text-slate-200">"leemark@don:~/contact$"</span>
                                                    <span class="text-emerald-600 dark:text-emerald-400 font-semibold ml-1">"ping -c 1 connection.socket"</span>
                                                </div>

                                                <p class="text-xs text-slate-600 dark:text-slate-300/90 font-sans pl-4 border-l-2 border-emerald-500/60 text-left">
                                                    "Have a question or want to work together? Feel free to reach out through any of the channels below."
                                                </p>

                                                <div class="flex flex-col sm:flex-row justify-center sm:justify-start items-center gap-6 text-sm font-semibold text-emerald-700 dark:text-emerald-400 pt-2 bg-white/70 dark:bg-black/40 p-4 rounded-xl border border-emerald-500/20 shadow-sm">
                                                    <a href="mailto:leemarkarojo7@gmail.com" class="hover:underline flex items-center gap-2">
                                                        <i class="fa-solid fa-envelope text-emerald-600 dark:text-emerald-300"></i> "leemarkarojo7@gmail.com"
                                                    </a>
                                                    <a href="tel:+639096830094" class="hover:underline flex items-center gap-2">
                                                        <i class="fa-solid fa-phone text-emerald-600 dark:text-emerald-300"></i> "+63 909 683 0094"
                                                    </a>
                                                    <span class="text-slate-500 dark:text-slate-400 flex items-center gap-2">
                                                        <i class="fa-solid fa-location-dot text-emerald-600 dark:text-emerald-500"></i> "Philippines"
                                                    </span>
                                                </div>

                                                // Active Terminal Line with Cursor
                                                <div class="flex items-center gap-1.5 pt-3">
                                                    <span class="text-slate-700 dark:text-slate-200 font-medium">"leemark@don:~/contact$"</span>
                                                    <span class="w-2.5 h-4 bg-slate-800 dark:bg-slate-100 inline-block animate-pulse ml-0.5"></span>
                                                </div>
                                            </div>
                                        </div>
                                    </section>
        </main>

        <footer class="text-center py-6 text-xs text-slate-500 border-t border-emerald-500/20 relative z-10">
            "© Leemark E. Arojo. All rights reserved."
        </footer>
    }
}

pub fn main() {
    console_error_panic_hook::set_once();

    let app_element = document()
        .get_element_by_id("app")
        .unwrap()
        .unchecked_into::<web_sys::HtmlElement>();

    leptos::mount::mount_to(app_element, App).forget();
}
