package io.github.zannabianca1997.apelle.search.services;

import org.jboss.logging.Logger;

import io.github.zannabianca1997.apelle.common.dtos.Page;
import io.github.zannabianca1997.apelle.common.dtos.PageRequest;
import io.github.zannabianca1997.apelle.search.dtos.SearchedSongQueryDto;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.github.zannabianca1997.apelle.users.services.UsersService;
import io.github.zannabianca1997.apelle.youtube.services.YoutubeService;
import jakarta.enterprise.context.ApplicationScoped;

@ApplicationScoped
public class SearchService {
    private final Logger log;

    private final YoutubeService youtubeService;

    private final UsersService usersService;

    public SearchService(final YoutubeService youtubeService, final UsersService usersService, final Logger log) {
        this.youtubeService = youtubeService;
        this.usersService = usersService;
        this.log = log;
    }

    public Page<SearchedSongQueryDto> search(final String query, final PageRequest pageRequest) {
        final ApelleUser current = usersService.getMe();

        log.infof("[user=%s] Searched `%s`", current.getId(), query);

        // Only source is youtube, delegating to it
        return youtubeService.search(query, pageRequest);
    }

}
