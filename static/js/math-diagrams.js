const MathDiagram = ( e => {

    const math = e => {
        MathJax.typesetPromise().then( e => {
            const spanContent = document.querySelector('.math-display').textContent.trim();
            
            document.querySelector('.math-display').replaceWith(
                MathJax.tex2chtml(spanContent, {
                    display: true
                })
            );
        });
    };

    const diagram = e => {
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

    math();

    diagram();

})();
