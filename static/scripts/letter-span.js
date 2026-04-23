function spanEachLetter() {
    document.querySelectorAll('.nav-pages li a').forEach(function(link) {
        link.innerHTML = link.textContent.split('').map(function(v, i) {
            return `<span style="animation-delay:${i * 0.1}s">${v}</span>`;
        }).join('');
    });
}

document.addEventListener('DOMContentLoaded', spanEachLetter);
