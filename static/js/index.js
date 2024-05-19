window.onload = e => {

    Citations.init();
    DocsSources.init();

    let pageTitle = document.title.split(':');
    document.getElementById('header-label').textContent = `${ pageTitle[1] }: ${ pageTitle[2] }`;
    
};