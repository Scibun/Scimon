const MathDiagram = ( e => {

    let math = e => {
        MathJax.typesetPromise().then( e => {
            document.querySelectorAll('.math-display').forEach( block => {
                block.replaceWith(
                    MathJax.tex2chtml(block.textContent.trim(), {
                        display: true
                    }).childNodes[0]
                );
            });
        });
    };

    let diagram = e => {
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

    let init = e => {
        let interval = setInterval( e => {
            math();
            diagram();
    
            if (document.getElementsByTagName('mjx-math').length > 0) {
                clearInterval(interval);
            }
        }, 100);
    };

    return {
        init: () => { return init(); },
        math: () => { return math(); },
        diagram: () => { return diagram(); },
    };

})();
