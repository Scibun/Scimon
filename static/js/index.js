window.onload = e => {

    Elements.pageTitle();

    Citations.init();
    MathDiagram.init();
    DocsSources.init();

    ScrollTo.checkScroll();
    window.addEventListener('scroll', ScrollTo.checkScroll);

    window.addEventListener("click", event => { BibTex.autoHideClickOffset(event); });

    document.getElementById(BibTex.code).addEventListener('click', BibTex.copy);

    document.getElementById(ScrollTo.topBtn).addEventListener('click', ScrollTo.top);
    document.getElementById(Citations.toggleBtn).addEventListener('click', Citations.scrollTo);
    document.getElementById(DocsSources.toggleBtn).addEventListener('click', DocsSources.scrollTo);
    
};
