# LiveDatadash
Viewing live data in style


# Web testing
1. Open http://localhost:3000/ in browser
2. Navigate to ./Vite_Fronted in terminal
3. Run 'npm run dev'
Changes will be made as soon as files are saved

# Goodies
- Use .svg for images, node.js dislikes other file types for optimization reasons
- To change tab image the file favicon.ico must be changed (I cannot find where it is called in the code)
- Tauri's JS APIs require access to browser-only APIs, meaning if you want to use one you have to change the entire file to only use client compoents or create a new file that uses file components (see greet.tsx as example)