const Mermaid = ( e => {

    const load = e => {
        mermaid.initialize({
            theme: 'dark',
            securityLevel: 'loose',
        });
    
        let mermaidCodeBlocks = document.querySelectorAll('.language-mermaid');
    
        mermaidCodeBlocks.forEach( block => {
            block.classList.remove('language-mermaid');
            block.classList.add('diagram-mermaid');
        });
    
        mermaidCodeBlocks.forEach( block => {
            let diagramDiv = document.createElement('div');
            diagramDiv.classList.add('mermaid');
            
            diagramDiv.textContent = block.textContent;
    
            block.textContent = '';
            block.appendChild(diagramDiv);
    
            mermaid.init(undefined, diagramDiv);
        });
    };

    load();

})()
