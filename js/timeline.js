document.addEventListener('DOMContentLoaded', function() {
    // Get timeline container
    const timeline = document.getElementById('timeline');

    // Calculate timeline width (1 year) in milliseconds
    const oneYearMs = 365 * 24 * 60 * 60 * 1000;

    // Get today's date
    const today = new Date();

    // Calculate start date (1 year ago)
    const startDate = new Date(today);
    startDate.setFullYear(today.getFullYear() - 1);

    // Filter out projects that were started over a year ago
    const filteredProjects = projects.filter(project => new Date(project.startDate) >= startDate);

    // Calculate number of projects and height of each project bar
    const numProjects = filteredProjects.length;
    const projectHeight = 100 / numProjects;

    // Loop through projects and create corresponding bars
    filteredProjects.forEach((project, index) => {
        // Calculate project duration, width, offset, and position
        const projectDuration = project.endDate - project.startDate;
        const projectWidth = (projectDuration / oneYearMs) * 100;
        const projectOffset = ((project.startDate - startDate) / oneYearMs) * 100;
        const projectPosition = index * projectHeight;

        // Create project bar
        const projectBar = document.createElement('div');

        // Add classes and styles
        projectBar.classList.add('project-bar');
        projectBar.style.width = `${projectWidth}%`;
        projectBar.style.top = `${projectPosition}%`;
        projectBar.style.left = `${projectOffset}%`;
        projectBar.style.height = `${projectHeight}%`;

        timeline.appendChild(projectBar);
    });

    // Create vertical lines for the first of every month
    const monthLines = document.createElement('div');
    monthLines.classList.add('month-lines');

    for (let i = 0; i < 12; i++) {
        const monthLine = document.createElement('div');
        monthLine.classList.add('month-line');
        monthLine.style.left = `${(i + 1) * (100 / 12)}%`;
        monthLines.appendChild(monthLine);
    }

    timeline.appendChild(monthLines);
});
