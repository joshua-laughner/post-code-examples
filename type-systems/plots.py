import matplotlib.pyplot as plt
import pandas as pd

plt.xkcd()
plt.rcParams['font.family'] = ['xkcd Script']

xmax=5.0


def time_plot(pytimes, rstimes, savename, title=None, xmax=xmax, clipping=True):
    _, ax = plt.subplots()
    index = [i for i, p in zip(['Initial', 'Testing', 'Maintenance'], pytimes) if p is not None]
    colors = {'Initial': 'tab:blue', 'Testing': 'tab:orange', 'Maintenance': 'tab:green'}
    colors = [colors[i] for i in index]
    df = pd.DataFrame({'Python': pytimes, 'Rust': rstimes}, index=index)
    df.T.plot.barh(stacked=True, color=colors, clip_on=clipping, ax=ax)
    ax.set_xlim(0, xmax)
    ax.set_xlabel('Time (arbitrary)')
    ax.legend(loc='center left', bbox_to_anchor=(1.01, 0.5))
    if title is not None:
        ax.set_title(title)
    plt.savefig(f'../../posts-markdown/static/posts/img/type-systems/{savename}.svg', bbox_inches='tight')


time_plot([0.5, 0.2], [1.0, 0.1], 'data-analysis', 'Data analysis')
time_plot([0.5, 0.2, 0.3], [1.0, 0.1, 0.3], 'data-analysis-tool', 'Data analysis tool')
time_plot([0.8, 0.8, 1.5], [1.6, 0.4, 0.4], 'data-tool-complex', 'More complicated data analysis tool')
time_plot([1.0, 1.5, 4.5], [2.0, 1.0, 1.0], 'model-or-similar', 'A very complicated community program', clipping=False)
